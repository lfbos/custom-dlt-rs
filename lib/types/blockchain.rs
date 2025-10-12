use super::{Block, Transaction, TransactionOutput};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::util::{MerkleRoot, Saveable};
use crate::U256;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    utxos: HashMap<Hash, (bool, TransactionOutput)>,
    target: U256,
    blocks: Vec<Block>,
    #[serde(default, skip_serializing)]
    mempool: Vec<(DateTime<Utc>, Transaction)>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            utxos: HashMap::new(),
            blocks: vec![],
            target: crate::MIN_TARGET,
            mempool: vec![],
        }
    }

    pub fn utxos(&self) -> &HashMap<Hash, (bool, TransactionOutput)> {
        &self.utxos
    }

    pub fn target(&self) -> U256 {
        self.target
    }

    pub fn blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }

    // block height
    pub fn block_height(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn mempool(&self) -> &[(DateTime<Utc>, Transaction)] {
        // later, we will also need to keep track of time
        &self.mempool
    }

    // Rebuild UTXO set from the blockchain
    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }

                for output in transaction.outputs.iter() {
                    self.utxos.insert(output.hash(), (false, output.clone()));
                }
            }
        }
    }

    /// Adds a transaction to the mempool after validation.
    ///
    /// This function implements Replace-By-Fee (RBF) logic by allowing new transactions
    /// to replace existing ones in the mempool if they try to spend the same UTXOs.
    ///
    /// # Validation Steps:
    /// 1. Verify all inputs reference existing UTXOs
    /// 2. Ensure no duplicate inputs within the transaction
    /// 3. Handle UTXO marking conflicts (RBF logic)
    /// 4. Verify input sum ≥ output sum
    /// 5. Mark UTXOs as "in use" by this mempool transaction
    /// 6. Sort mempool by fee (highest first)
    ///
    /// # UTXO Marking System:
    /// Each UTXO in the HashMap has a boolean flag:
    /// - false: UTXO is unspent and not reserved by any mempool transaction
    /// - true: UTXO is reserved by a pending transaction in mempool
    ///
    /// This prevents wallets from creating conflicting transactions.
    pub fn add_to_mempool(&mut self, transaction: Transaction) -> Result<()> {
        // STEP 1: Basic validation - check all inputs exist and are unique
        // =================================================================
        // We need to ensure:
        // a) Every input references a real UTXO
        // b) No input is used twice in the same transaction (internal double-spend)
        let mut known_inputs: HashSet<Hash> = HashSet::new();
        for input in &transaction.inputs {
            // Check UTXO exists in our set
            if !self.utxos.contains_key(&input.prev_transaction_output_hash) {
                return Err(BtcError::InvalidTransaction);
            }
            // Check this input isn't duplicated
            if known_inputs.contains(&input.prev_transaction_output_hash) {
                return Err(BtcError::InvalidTransaction);
            }
            known_inputs.insert(input.prev_transaction_output_hash);
        }

        // STEP 2: Handle Replace-By-Fee (RBF) logic
        // ==========================================
        // If any UTXO we're trying to spend is already marked (reserved by another
        // mempool transaction), we implement RBF: remove the old transaction and
        // accept the new one.
        //
        // Example scenario:
        // - Alice creates Transaction A using UTXO #1
        // - Transaction A enters mempool, UTXO #1 is marked
        // - Alice creates Transaction B also using UTXO #1 (with higher fee)
        // - We remove Transaction A from mempool and unmark its UTXOs
        // - Transaction B replaces it
        for input in &transaction.inputs {
            if let Some((true, _)) = self.utxos.get(&input.prev_transaction_output_hash) {
                // This UTXO is already marked - find which mempool transaction has it
                // We search for a transaction whose OUTPUT hash matches our INPUT hash
                let referencing_transaction =
                    self.mempool.iter().enumerate().find(|(_, (_, tx))| {
                        tx.outputs
                            .iter()
                            .any(|output| output.hash() == input.prev_transaction_output_hash)
                    });

                // Found the conflicting transaction - remove it and unmark all its UTXOs
                if let Some((idx, (_, referencing_transaction))) = referencing_transaction {
                    for input in &referencing_transaction.inputs {
                        // Unmark all UTXOs that the old transaction was trying to spend
                        self.utxos
                            .entry(input.prev_transaction_output_hash)
                            .and_modify(|(marked, _)| {
                                *marked = false;
                            });
                    }
                    // Remove the old transaction from mempool (it's being replaced)
                    self.mempool.remove(idx);
                } else {
                    // Edge case: UTXO is marked but we can't find the transaction
                    // This shouldn't happen, but we handle it gracefully by unmarking
                    self.utxos
                        .entry(input.prev_transaction_output_hash)
                        .and_modify(|(marked, _)| {
                            *marked = false;
                        });
                }
            }
        }
        // STEP 3: Economic validation - verify transaction is financially valid
        // ======================================================================
        // The sum of all inputs must be ≥ sum of all outputs
        // The difference is the transaction fee for the miner
        //
        // Example:
        // Inputs: [10 BTC, 5 BTC] = 15 BTC total
        // Outputs: [12 BTC, 2.99 BTC] = 14.99 BTC total
        // Fee: 15 - 14.99 = 0.01 BTC (goes to miner)
        let all_inputs = transaction
            .inputs
            .iter()
            .map(|input| {
                self.utxos
                    .get(&input.prev_transaction_output_hash)
                    .expect("BUG: impossible - we validated this exists above")
                    .1
                    .value
            })
            .sum::<u64>();
        let all_outputs = transaction
            .outputs
            .iter()
            .map(|output| output.value)
            .sum::<u64>();

        if all_inputs < all_outputs {
            print!("inputs are lower than outputs");
            return Err(BtcError::InvalidTransaction);
        }

        // STEP 4: Mark UTXOs as reserved by this transaction
        // ===================================================
        // Set the boolean flag to true for each UTXO this transaction spends
        // This prevents double-spending within the mempool
        for input in &transaction.inputs {
            self.utxos
                .entry(input.prev_transaction_output_hash)
                .and_modify(|(marked, _)| {
                    *marked = true;
                });
        }

        // STEP 5: Add to mempool with timestamp
        // ======================================
        // Timestamp is used for cleanup (removing old transactions)
        self.mempool.push((Utc::now(), transaction));

        // STEP 6: Sort mempool by transaction fee (highest first)
        // ========================================================
        // Miners will prefer transactions with higher fees
        // This prioritization happens every time a transaction is added
        //
        // Note: This is inefficient (O(n log n) on every insert)
        // Production systems use priority queues instead
        self.mempool.sort_by_key(|(_, tx)| {
            // Calculate fee for this transaction
            let all_inputs = tx
                .inputs
                .iter()
                .map(|input| {
                    self.utxos
                        .get(&input.prev_transaction_output_hash)
                        .unwrap()
                        .1
                        .value
                })
                .sum::<u64>();
            let all_outputs = tx.outputs.iter().map(|output| output.value).sum::<u64>();
            let miner_fee = all_inputs - all_outputs;
            miner_fee
        });
        Ok(())
    }

    // try to add a new block to the blockchain,
    // return an error if it is not valid to insert this
    // block to this blockchain
    pub fn add_block(&mut self, block: Block) -> Result<()> {
        // check if the block is valid
        if self.blocks.is_empty() {
            // if this is the first block, check if the
            // block's prev_block_hash is all zeroes
            if block.header.prev_block_hash != Hash::zero() {
                println!("zero hash");
                return Err(BtcError::InvalidBlock);
            }
        } else {
            // if this is not the first block, check if the
            // block's prev_block_hash is the hash of the last block
            let last_block = self.blocks.last().unwrap();
            if block.header.prev_block_hash != last_block.hash() {
                println!("prev hash is wrong");
                return Err(BtcError::InvalidBlock);
            }
            // check if the block's hash is less than the target
            if !block.header.hash().matches_target(block.header.target) {
                println!("does not match target");
                return Err(BtcError::InvalidBlock);
            }

            // check if the block's merkle root is correct
            let calculated_merkle_root = MerkleRoot::calculate(&block.transactions);
            if calculated_merkle_root != block.header.merkle_root {
                println!("invalid merkle root");
                return Err(BtcError::InvalidMerkleRoot);
            }

            // check if the block's timestamp is after the
            // last block's timestamp
            if block.header.timestamp <= last_block.header.timestamp {
                return Err(BtcError::InvalidBlock);
            }
            // Verify all transactions in the block
            block.verify_transactions(self.block_height(), &self.utxos)?;
        }
        // Remove transactions from mempool that are now in the block
        let block_transactions: HashSet<_> =
            block.transactions.iter().map(|tx| tx.hash()).collect();
        self.mempool
            .retain(|(_, tx)| !block_transactions.contains(&tx.hash()));
        self.blocks.push(block);
        self.try_adjust_target();
        Ok(())
    }

    /// Adjusts the mining difficulty target to maintain consistent block times.
    ///
    /// This function implements Bitcoin's difficulty adjustment algorithm. It runs
    /// every DIFFICULTY_UPDATE_INTERVAL blocks (50 blocks) and adjusts the target
    /// based on how fast the last 50 blocks were mined.
    ///
    /// # Algorithm:
    ///
    /// ```text
    /// new_target = current_target × (actual_time / target_time)
    /// ```
    ///
    /// # Example:
    ///
    /// Target: 500 seconds for 50 blocks (10 seconds per block)
    ///
    /// **Case 1: Blocks mined too fast (more mining power)**
    /// - Actual time: 250 seconds (5 seconds per block)
    /// - new_target = current_target × (250 / 500) = current_target × 0.5
    /// - Target becomes smaller (HARDER difficulty)
    ///
    /// **Case 2: Blocks mined too slow (less mining power)**
    /// - Actual time: 1000 seconds (20 seconds per block)
    /// - new_target = current_target × (1000 / 500) = current_target × 2
    /// - Target becomes larger (EASIER difficulty)
    ///
    /// # Safety Limits:
    /// - Maximum adjustment: 4x easier or 4x harder per adjustment
    /// - Never easier than MIN_TARGET (maximum difficulty floor)
    pub fn try_adjust_target(&mut self) {
        // Early return if blockchain is empty
        if self.blocks.is_empty() {
            return;
        }

        // Only adjust every DIFFICULTY_UPDATE_INTERVAL blocks (e.g., every 50 blocks)
        if self.blocks.len() % crate::DIFFICULTY_UPDATE_INTERVAL as usize != 0 {
            return;
        }

        // STEP 1: Measure actual time taken for last adjustment interval
        // ==============================================================
        // Get the timestamp of the block that started this interval
        let start_time = self.blocks
            [self.blocks.len() - crate::DIFFICULTY_UPDATE_INTERVAL as usize]
            .header
            .timestamp;
        
        // Get the timestamp of the most recent block
        let end_time = self.blocks.last().unwrap().header.timestamp;
        
        // Calculate the actual time difference
        let time_diff = end_time - start_time;
        let time_diff_seconds = time_diff.num_seconds();

        // STEP 2: Calculate target (ideal) time
        // ======================================
        // We want IDEAL_BLOCK_TIME (10 seconds) per block
        // Over DIFFICULTY_UPDATE_INTERVAL blocks, that's:
        // 10 seconds/block × 50 blocks = 500 seconds total
        let target_seconds = crate::IDEAL_BLOCK_TIME * crate::DIFFICULTY_UPDATE_INTERVAL;

        // STEP 3: Calculate new target with adjustment ratio
        // ===================================================
        // Formula: new_target = current_target × (actual_time / target_time)
        //
        // We use BigDecimal for precision since U256 doesn't support division
        let new_target = BigDecimal::parse_bytes(&self.target.to_string().as_bytes(), 10)
            .expect("BUG: impossible")
            * (BigDecimal::from(time_diff_seconds) / BigDecimal::from(target_seconds));

        // STEP 4: Convert back to U256
        // =============================
        // Truncate decimal places (we only need the integer part)
        let new_target_str = new_target
            .to_string()
            .split('.')
            .next()
            .expect("BUG: Expected a decimal point")
            .to_owned();
        let new_target: U256 = U256::from_str_radix(&new_target_str, 10).expect("BUG: impossible");

        // STEP 5: Apply safety clamps
        // ============================
        // Prevent extreme difficulty swings by limiting adjustment to 4x in either direction
        // This prevents a single adjustment from making mining impossibly hard or trivially easy
        let new_target = if new_target < self.target / 4 {
            // Don't make it more than 4x harder
            self.target / 4
        } else if new_target > self.target * 4 {
            // Don't make it more than 4x easier
            self.target * 4
        } else {
            new_target
        };

        // STEP 6: Apply absolute maximum (difficulty floor)
        // ==================================================
        // Never allow target to exceed MIN_TARGET (the easiest allowed difficulty)
        self.target = new_target.min(crate::MIN_TARGET);
    }

    // Cleanup mempool - remove transactions older than
    // MAX_MEMPOOL_TRANSACTION_AGE
    pub fn cleanup_mempool(&mut self) {
        let now = Utc::now();
        let mut utxo_hashes_to_unmark: Vec<Hash> = vec![];
        self.mempool.retain(|(timestamp, transaction)| {
            if now - *timestamp
                > chrono::Duration::seconds(crate::MAX_MEMPOOL_TRANSACTION_AGE as i64)
            {
                // push all utxos to unmark to the vector
                // so we can unmark them later
                utxo_hashes_to_unmark.extend(
                    transaction
                        .inputs
                        .iter()
                        .map(|input| input.prev_transaction_output_hash),
                );
                false
            } else {
                true
            }
        });
        // unmark all of the UTXOs
        for hash in utxo_hashes_to_unmark {
            self.utxos.entry(hash).and_modify(|(marked, _)| {
                *marked = false;
            });
        }
    }
    pub fn calculate_block_reward(&self) -> u64 {
        let block_height = self.block_height();
        let halvings = block_height / crate::HALVING_INTERVAL;
        (crate::INITIAL_REWARD * 10u64.pow(8)) >> halvings
    }
}

impl Saveable for Blockchain {
    fn load<I: Read>(reader: I) -> IoResult<Self> {
        ciborium::de::from_reader(reader)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to deserialize Blockchain"))
    }

    fn save<O: Write>(&self, writer: O) -> IoResult<()> {
        ciborium::ser::into_writer(self, writer)
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to serialize Blockchain"))
    }
}
