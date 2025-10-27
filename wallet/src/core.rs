use anyhow::Result;
use btclib::crypto::{PrivateKey, PublicKey, Signature};
use btclib::network::Message;
use btclib::types::{Transaction, TransactionOutput};
use btclib::util::Saveable;
use crossbeam_skiplist::SkipMap;
use kanal::Sender;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// Represent a key pair with paths to public and private keys.
#[derive(Serialize, Deserialize, Clone)]
pub struct Key {
    pub public: PathBuf,
    pub private: PathBuf,
}
/// Represent a recipient with a name and a path to their public key.
#[derive(Serialize, Deserialize, Clone)]
pub struct Recipient {
    pub name: String,
    pub key: PathBuf,
}

/// Represent a loaded recipient with their actual public key.
#[derive(Clone)]
struct LoadedKey {
    public: PublicKey,
    private: PrivateKey,
}

#[derive(Clone)]
pub struct LoadedRecipient {
    #[allow(dead_code)]
    pub name: String,
    pub key: PublicKey,
}

impl Recipient {
    pub fn load(&self) -> Result<LoadedRecipient> {
        let key = PublicKey::load_from_file(&self.key)?;
        Ok(LoadedRecipient {
            name: self.name.clone(),
            key,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FeeType {
    Fixed,
    Percent,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FeeConfig {
    pub fee_type: FeeType,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub my_keys: Vec<Key>,
    pub contacts: Vec<Recipient>,
    pub default_node: String,
    pub fee_config: FeeConfig,
}

#[derive(Clone)]
struct UtxoStore {
    my_keys: Vec<LoadedKey>,
    utxos: Arc<SkipMap<PublicKey, Vec<(bool, TransactionOutput)>>>,
}

impl UtxoStore {
    fn new() -> Self {
        Self {
            my_keys: vec![],
            utxos: Arc::new(SkipMap::new()),
        }
    }
    fn add_key(&mut self, key: LoadedKey) {
        self.my_keys.push(key);
    }
}

#[derive(Clone)]
pub struct Core {
    pub config: Config,
    utxos: UtxoStore,
    pub tx_sender: Sender<Transaction>,
    pub stream: Arc<Mutex<TcpStream>>,
}

impl Core {
    fn new(config: Config, utxos: UtxoStore, stream: TcpStream) -> Self {
        let (tx_sender, _) = kanal::bounded(10);
        Core {
            config,
            utxos,
            tx_sender,
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    /// Load the Core from a configuration file
    pub async fn load(config_path: PathBuf) -> Result<Self> {
        info!("Loading core from config: {:?}", config_path);
        let config: Config = toml::from_str(&fs::read_to_string(&config_path)?)?;
        let mut utxos = UtxoStore::new();
        let stream = TcpStream::connect(&config.default_node).await?;
        // Load keys from config
        for key in &config.my_keys {
            debug!("Loading key pair: {:?}", key.public);
            let public = PublicKey::load_from_file(&key.public)?;
            let private = PrivateKey::load_from_file(&key.private)?;
            utxos.add_key(LoadedKey { public, private });
        }
        Ok(Core::new(config, utxos, stream))
    }

    /// Fetch UTXOs from the node for all loaded keys.
    pub async fn fetch_utxos(&self) -> Result<()> {
        debug!("Fetching UTXOs from node: {}", self.config.default_node);
        for key in &self.utxos.my_keys {
            let message = Message::FetchUTXOs(key.public.clone());
            message.send_async(&mut *self.stream.lock().await).await?;
            if let Message::UTXOs(utxos) =
                Message::receive_async(&mut *self.stream.lock().await).await?
            {
                debug!("Received {} UTXOs for key: {:?}", utxos.len(), key.public);
                // Replace the entire UTXO set for this key
                self.utxos.utxos.insert(
                    key.public.clone(),
                    utxos
                        .into_iter()
                        .map(|(output, marked)| (marked, output))
                        .collect(),
                );
            } else {
                error!("Unexpected response from node");
                return Err(anyhow::anyhow!("Unexpected response from node"));
            }
        }
        info!("UTXOs fetched successfully");
        Ok(())
    }

    /// Send a transaction to the node.
    pub async fn send_transaction(&self, transaction: Transaction) -> Result<()> {
        debug!("Sending transaction to node: {}", self.config.default_node);
        let message = Message::SubmitTransaction(transaction);
        message.send_async(&mut *self.stream.lock().await).await?;
        info!("Transaction sent successfully");
        Ok(())
    }

    /// Prepare and send a transaction asynchronously.
    pub fn send_transaction_async(&self, recipient: &str, amount: u64) -> Result<()> {
        info!("Preparing to send {} satoshis to {}", amount, recipient);
        let recipient_key = self
            .config
            .contacts
            .iter()
            .find(|r| r.name == recipient)
            .ok_or_else(|| anyhow::anyhow!("Recipient not found"))?
            .load()?
            .key;
        let transaction = self.create_transaction(&recipient_key, amount)?;
        debug!("Sending transaction asynchronously");
        self.tx_sender.send(transaction)?;
        Ok(())
    }

    /// Creates a transaction by selecting UTXOs and generating signatures.
    ///
    /// This function implements a simple greedy coin selection algorithm:
    /// it iterates through available UTXOs and adds them to the transaction
    /// until the required amount (payment + fee) is covered.
    ///
    /// # Coin Selection Algorithm:
    ///
    /// ```text
    /// Goal: Send 10 BTC with 0.1 BTC fee (need 10.1 BTC total)
    ///
    /// Available UTXOs:
    /// - UTXO A: 3 BTC
    /// - UTXO B: 5 BTC  
    /// - UTXO C: 8 BTC
    ///
    /// Selection process:
    /// 1. Add UTXO A: 3 BTC (total: 3, need: 10.1) - not enough
    /// 2. Add UTXO B: 5 BTC (total: 8, need: 10.1) - not enough
    /// 3. Add UTXO C: 8 BTC (total: 16, need: 10.1) - enough!
    ///
    /// Transaction created:
    /// Inputs: [UTXO A, UTXO B, UTXO C] = 16 BTC
    /// Outputs:
    ///   - 10 BTC → recipient
    ///   - 5.9 BTC → self (change)
    /// Fee: 0.1 BTC (implicit, goes to miner)
    /// ```
    ///
    /// # Arguments
    /// * `recipient` - Public key of the recipient
    /// * `amount` - Amount to send in satoshis
    ///
    /// # Returns
    /// * `Ok(Transaction)` - A signed transaction ready to broadcast
    /// * `Err` - If insufficient funds or signing fails
    pub fn create_transaction(&self, recipient: &PublicKey, amount: u64) -> Result<Transaction> {
        // STEP 1: Calculate total amount needed (payment + fee)
        let fee = self.calculate_fee(amount);
        let total_amount = amount + fee;

        // STEP 2: Coin selection - gather enough UTXOs using greedy algorithm
        let mut inputs = Vec::new();
        let mut input_sum = 0;

        // Iterate through all our UTXOs across all keys
        for entry in self.utxos.utxos.iter() {
            let pubkey = entry.key();
            let utxos = entry.value();

            for (marked, utxo) in utxos.iter() {
                // Skip UTXOs reserved by pending mempool transactions
                if *marked {
                    continue;
                }

                // Stop if we already have enough
                if input_sum >= total_amount {
                    break;
                }

                // Add this UTXO as input and sign it with the corresponding private key
                inputs.push(btclib::types::TransactionInput {
                    prev_transaction_output_hash: utxo.hash(),
                    signature: Signature::sign_output(
                        &utxo.hash(),
                        &mut self
                            .utxos
                            .my_keys
                            .iter()
                            .find(|k| k.public == *pubkey)
                            .unwrap()
                            .private
                            .clone(),
                    ),
                });
                input_sum += utxo.value;
            }

            // Check if we've collected enough across all keys
            if input_sum >= total_amount {
                break;
            }
        }

        // STEP 3: Verify we have sufficient funds
        if input_sum < total_amount {
            return Err(anyhow::anyhow!("Insufficient funds"));
        }

        // STEP 4: Create outputs (payment to recipient)
        let mut outputs = vec![TransactionOutput {
            value: amount,
            unique_id: uuid::Uuid::new_v4(),
            pubkey: recipient.clone(),
        }];

        // STEP 5: Add change output if we have excess (send back to ourselves)
        if input_sum > total_amount {
            outputs.push(TransactionOutput {
                value: input_sum - total_amount,
                unique_id: uuid::Uuid::new_v4(),
                pubkey: self.utxos.my_keys[0].public.clone(),
            });
        }

        // STEP 6: Return the completed, signed transaction
        Ok(Transaction { inputs, outputs })
    }

    pub fn get_balance(&self) -> u64 {
        let balance = self
            .utxos
            .utxos
            .iter()
            .map(|entry| {
                let total_for_key = entry
                    .value()
                    .iter()
                    .filter(|(marked, _)| !*marked) // Exclude marked UTXOs (already being spent)
                    .map(|(_, utxo)| utxo.value)
                    .sum::<u64>();
                debug!("Balance for key: {} satoshis", total_for_key);
                total_for_key
            })
            .sum();
        debug!(
            "Total balance: {} satoshis ({} BTC)",
            balance,
            balance as f64 / 100_000_000.0
        );
        balance
    }

    fn calculate_fee(&self, amount: u64) -> u64 {
        match self.config.fee_config.fee_type {
            FeeType::Fixed => self.config.fee_config.value as u64,
            FeeType::Percent => (amount as f64 * self.config.fee_config.value / 100.0) as u64,
        }
    }
}
