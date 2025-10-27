use btclib::config;
use btclib::network::Message;
use btclib::sha256::Hash;
use btclib::types::{Block, BlockHeader, Transaction, TransactionOutput};
use btclib::util::MerkleRoot;
use chrono::Utc;
use tokio::net::TcpStream;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

pub async fn handle_connection(mut socket: TcpStream) {
    loop {
        // read a message from the socket
        let message = match Message::receive_async(&mut socket).await {
            Ok(message) => message,
            Err(e) => {
                warn!("invalid message from peer: {e}, closing that connection");
                return;
            }
        };

        use btclib::network::Message::*;
        match message {
            UTXOs(_) | Template(_) | Difference(_) | TemplateValidity(_) | NodeList(_) => {
                error!("I am neither a miner nor a wallet! Goodbye");
                return;
            }
            FetchBlock(height) => {
                // Clone the block first, then release lock before network I/O
                let block = {
                    let blockchain = crate::BLOCKCHAIN.read().await;
                    let x = blockchain.blocks().nth(height as usize).cloned();
                    x
                };
                let Some(block) = block else {
                    return;
                };
                // Lock is now released - safe to do network I/O
                let message = NewBlock(block);
                message.send_async(&mut socket).await.unwrap();
            }
            DiscoverNodes => {
                let nodes = crate::NODES
                    .iter()
                    .map(|x| x.key().clone())
                    .collect::<Vec<_>>();
                let message = NodeList(nodes);
                message.send_async(&mut socket).await.unwrap();
            }
            AskDifference(height) => {
                // Get block height immediately and release lock
                let count = {
                    let blockchain = crate::BLOCKCHAIN.read().await;
                    blockchain.block_height() as i32 - height as i32
                };
                let message = Difference(count);
                message.send_async(&mut socket).await.unwrap();
            }
            FetchUTXOs(key) => {
                debug!("received request to fetch UTXOs");
                // Collect UTXOs immediately and release lock
                let utxos = {
                    let blockchain = crate::BLOCKCHAIN.read().await;
                    blockchain
                        .utxos()
                        .iter()
                        .filter(|(_, (_, txout))| txout.pubkey == key)
                        .map(|(_, (marked, txout))| (txout.clone(), *marked))
                        .collect::<Vec<_>>()
                };
                let message = UTXOs(utxos);
                message.send_async(&mut socket).await.unwrap();
            }
            NewBlock(block) => {
                // Acquire write lock only for the blockchain operation
                let result = {
                    let mut blockchain = crate::BLOCKCHAIN.write().await;
                    info!("received new block");
                    blockchain.add_block(block)
                };
                if let Err(e) = result {
                    warn!("block rejected: {}", e);
                }
            }
            NewTransaction(tx) => {
                // Acquire write lock only for the mempool operation
                let result = {
                    let mut blockchain = crate::BLOCKCHAIN.write().await;
                    debug!("received transaction from friend");
                    blockchain.add_to_mempool(tx)
                };
                if let Err(e) = result {
                    warn!("transaction rejected, closing connection: {}", e);
                    return;
                }
            }
            ValidateTemplate(block_template) => {
                // Get last block hash immediately and release lock
                let status = {
                    let blockchain = crate::BLOCKCHAIN.read().await;
                    block_template.header.prev_block_hash
                        == blockchain
                            .blocks()
                            .last()
                            .map(|last_block| last_block.hash())
                            .unwrap_or(Hash::zero())
                };
                let message = TemplateValidity(status);
                message.send_async(&mut socket).await.unwrap();
            }
            SubmitTemplate(block) => {
                info!("received allegedly mined template");
                // Acquire write lock only for blockchain operations, then release before network I/O
                let block_clone = block.clone();
                let was_accepted = {
                    let mut blockchain = crate::BLOCKCHAIN.write().await;
                    match blockchain.add_block(block.clone()) {
                        Ok(_) => {
                            blockchain.rebuild_utxos();
                            true
                        }
                        Err(e) => {
                            warn!("block rejected: {}, closing connection", e);
                            false
                        }
                    }
                };

                if !was_accepted {
                    return;
                }

                info!("block looks good, broadcasting");
                // send block to all friend nodes - lock is now released
                let nodes = crate::NODES
                    .iter()
                    .map(|x| x.key().clone())
                    .collect::<Vec<_>>();
                for node in nodes {
                    if let Some(mut stream) = crate::NODES.get_mut(&node) {
                        let message = Message::NewBlock(block_clone.clone());
                        if message.send_async(&mut *stream).await.is_err() {
                            warn!("failed to send block to {}", node);
                        }
                    }
                }
            }
            SubmitTransaction(tx) => {
                debug!("submit tx");
                // Acquire write lock only for mempool operation, then release before network I/O
                let tx_clone = tx.clone();
                let result = {
                    let mut blockchain = crate::BLOCKCHAIN.write().await;
                    blockchain.add_to_mempool(tx)
                };

                if let Err(e) = result {
                    warn!("transaction rejected, closing connection: {}", e);
                    return;
                }

                debug!("added transaction to mempool");
                // send transaction to all friend nodes - lock is now released
                let nodes = crate::NODES
                    .iter()
                    .map(|x| x.key().clone())
                    .collect::<Vec<_>>();
                for node in nodes {
                    debug!("sending to friend: {node}");
                    if let Some(mut stream) = crate::NODES.get_mut(&node) {
                        let message = Message::SubmitTransaction(tx_clone.clone());
                        if message.send_async(&mut *stream).await.is_err() {
                            warn!("failed to send transaction to {}", node);
                        }
                    }
                }
                info!("transaction sent to friends");
            }
            FetchTemplate(pubkey) => {
                // Collect all necessary data and release lock before any expensive operations
                let (mempool_txs, prev_block_hash, target, utxos, reward) = {
                    let blockchain = crate::BLOCKCHAIN.read().await;
                    let mempool_txs = blockchain
                        .mempool()
                        .iter()
                        .take(config::block_transaction_cap())
                        .map(|(_, tx)| tx)
                        .cloned()
                        .collect::<Vec<_>>();
                    let prev_block_hash = blockchain
                        .blocks()
                        .last()
                        .map(|last_block| last_block.hash())
                        .unwrap_or(Hash::zero());
                    let target = blockchain.target();
                    let utxos = blockchain.utxos().clone();
                    let reward = blockchain.calculate_block_reward();
                    (mempool_txs, prev_block_hash, target, utxos, reward)
                };

                // Now build template without holding the lock
                let mut transactions = vec![];
                transactions.extend(mempool_txs);
                // insert coinbase tx with pubkey
                transactions.insert(
                    0,
                    Transaction {
                        inputs: vec![],
                        outputs: vec![TransactionOutput {
                            pubkey,
                            unique_id: Uuid::new_v4(),
                            value: 0,
                        }],
                    },
                );
                let merkle_root = MerkleRoot::calculate(&transactions);
                let mut block = Block::new(
                    BlockHeader {
                        timestamp: Utc::now(),
                        prev_block_hash,
                        nonce: 0,
                        target,
                        merkle_root,
                    },
                    transactions,
                );
                let miner_fees = match block.calculate_miner_fees(&utxos) {
                    Ok(fees) => fees,
                    Err(e) => {
                        error!("failed to calculate miner fees: {}", e);
                        return;
                    }
                };
                // update coinbase tx with reward
                block.transactions[0].outputs[0].value = reward + miner_fees;
                // recalculate merkle root
                block.header.merkle_root = MerkleRoot::calculate(&block.transactions);
                let message = Template(block);
                message.send_async(&mut socket).await.unwrap();
            }
        };
    }
}
