use anyhow::Result;
use argh::FromArgs;
use btclib::config::BlockchainConfig;
use btclib::types::Blockchain;
use dashmap::DashMap;
use static_init::dynamic;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

mod handler;
mod util;

#[dynamic]
pub static BLOCKCHAIN: RwLock<Blockchain> = RwLock::new(Blockchain::new());

#[dynamic]
pub static NODES: DashMap<String, TcpStream> = DashMap::new();

#[derive(FromArgs)]
/// A toy blockchain node
struct Args {
    #[argh(option)]
    /// port number (defaults to NODE_PORT env var or 9000)
    port: Option<u16>,
    #[argh(option)]
    /// blockchain file location (defaults to BLOCKCHAIN_FILE env var or ./blockchain.cbor)
    blockchain_file: Option<String>,
    #[argh(positional)]
    /// addresses of initial nodes (can also use INITIAL_PEERS env var)
    nodes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment
    let config = BlockchainConfig::global();
    
    // Parse command line arguments
    let args: Args = argh::from_env();
    
    // Priority: CLI args > Environment vars > Defaults
    let port = args.port.unwrap_or(config.node.port);
    let blockchain_file = args.blockchain_file
        .unwrap_or_else(|| config.node.blockchain_file.clone());
    
    // Combine CLI nodes with env var nodes
    let mut nodes = args.nodes;
    if nodes.is_empty() {
        nodes = config.node.initial_peers.clone();
    }
    
    println!("🚀 Starting blockchain node");
    println!("Network: {}", config.network.network_id);
    println!("Port: {}", port);
    println!("Blockchain file: {}", blockchain_file);
    if !nodes.is_empty() {
        println!("Initial peers: {:?}", nodes);
    }

    // Check if the blockchain_file exists
    if Path::new(&blockchain_file).exists() {
        util::load_blockchain(&blockchain_file).await?;
    } else {
        println!("blockchain file does not exist!");
        util::populate_connections(&nodes).await?;
        println!("total amount of known nodes: {}", NODES.len());
        if nodes.is_empty() {
            println!("no initial nodes provided, starting as a seed node");
        } else {
            let (longest_name, longest_count) = util::find_longest_chain_node().await?;
            // request the blockchain from the node with the longest blockchain
            util::download_blockchain(&longest_name, longest_count).await?;
            println!("blockchain downloaded from {}", longest_name);
            // recalculate utxos
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.rebuild_utxos();
            }
            // try to adjust difficulty
            {
                let mut blockchain = BLOCKCHAIN.write().await;
                blockchain.try_adjust_target();
            }
        }
    }

    // Start the TCP listener on 0.0.0.0:port
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);

    // start a task to periodically cleanup the mempool
    // normally, you would want to keep and join the handle
    tokio::spawn(util::cleanup());
    // and a task to periodically save the blockchain
    tokio::spawn(util::save(blockchain_file.clone()));
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handler::handle_connection(socket));
    }
}
