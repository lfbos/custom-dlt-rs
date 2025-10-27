use anyhow::{Context, Result};
use btclib::config::BlockchainConfig;
use btclib::network::Message;
use btclib::types::Blockchain;
use btclib::util::Saveable;
use tokio::net::TcpStream;
use tokio::time;
use tracing::info;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}

pub async fn load_blockchain(blockchain_file: &str) -> Result<()> {
    info!("blockchain file exists, loading...");
    let new_blockchain = Blockchain::load_from_file(blockchain_file)
        .context("Failed to load blockchain from file")?;
    info!("blockchain loaded");
    let mut blockchain = crate::BLOCKCHAIN.write().await;
    *blockchain = new_blockchain;
    info!("rebuilding utxos...");
    blockchain.rebuild_utxos();
    info!("utxos rebuilt");
    info!("checking if target needs to be adjusted...");
    info!("current target: {}", blockchain.target());
    blockchain.try_adjust_target();
    info!("new target: {}", blockchain.target());
    info!("initialization complete");
    Ok(())
}

pub async fn populate_connections(nodes: &[String]) -> Result<()> {
    info!("trying to connect to other nodes...");
    for node in nodes {
        let mut stream = TcpStream::connect(&node).await?;
        let message = Message::DiscoverNodes;
        message.send_async(&mut stream).await?;
        info!("sent DiscoverNodes to {}", node);
        let message = Message::receive_async(&mut stream).await?;

        match message {
            Message::NodeList(child_nodes) => {
                info!("received NodeList from {}", node);
                for child_node in child_nodes {
                    info!("adding node {}", child_node);
                    let new_stream = TcpStream::connect(&child_node).await?;
                    crate::NODES.insert(child_node, new_stream);
                }
            }
            _ => {
                info!("unexpected message from {}", node);
            }
        }
        crate::NODES.insert(node.clone(), stream);
    }
    Ok(())
}

pub async fn find_longest_chain_node() -> Result<(String, u32)> {
    info!("finding nodes with the highest blockchain length...");
    let mut longest_name = String::new();
    let mut longest_count = 0;
    let all_nodes = crate::NODES
        .iter()
        .map(|x| x.key().clone())
        .collect::<Vec<_>>();
    for node in all_nodes {
        info!("asking {} for blockchain length", node);
        let mut stream = crate::NODES.get_mut(&node).context("no node")?;
        let message = Message::AskDifference(0);
        message.send_async(&mut *stream).await.unwrap();
        info!("sent AskDifference to {}", node);
        let message = Message::receive_async(&mut *stream).await?;

        match message {
            Message::Difference(count) => {
                info!("received Difference from {}", node);
                if count > longest_count {
                    info!("new longest blockchain: {} blocks from {node}", count);
                    longest_count = count;
                    longest_name = node;
                }
            }
            e => {
                info!("unexpected message from {}: {:?}", node, e);
            }
        }
    }
    Ok((longest_name, longest_count as u32))
}

pub async fn download_blockchain(node: &str, count: u32) -> Result<()> {
    let mut stream = crate::NODES.get_mut(node).unwrap();
    for i in 0..count as usize {
        let message = Message::FetchBlock(i);
        message.send_async(&mut *stream).await?;
        let message = Message::receive_async(&mut *stream).await?;
        match message {
            Message::NewBlock(block) => {
                let mut blockchain = crate::BLOCKCHAIN.write().await;
                blockchain.add_block(block)?;
            }
            _ => {
                info!("unexpected message from {}", node);
            }
        }
    }
    Ok(())
}

pub async fn cleanup() {
    let config = BlockchainConfig::global();
    let mut interval = time::interval(time::Duration::from_secs(
        config.node.mempool_cleanup_interval_secs,
    ));
    loop {
        interval.tick().await;
        info!("cleaning the mempool from old transactions");
        let mut blockchain = crate::BLOCKCHAIN.write().await;
        blockchain.cleanup_mempool();
    }
}

pub async fn save(name: String) {
    let config = BlockchainConfig::global();
    let mut interval = time::interval(time::Duration::from_secs(
        config.node.blockchain_save_interval_secs,
    ));
    loop {
        interval.tick().await;
        info!("saving blockchain to drive...");
        let blockchain = crate::BLOCKCHAIN.read().await;
        blockchain.save_to_file(name.clone()).unwrap();
    }
}
