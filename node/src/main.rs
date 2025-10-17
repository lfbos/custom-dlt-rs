use anyhow::Result;
use clap::Parser;
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

#[derive(Parser)]
#[command(author, version, about = "A toy blockchain node", long_about = None)]
struct Args {
    /// Port number to listen on
    #[arg(short, long, env = "NODE_PORT")]
    port: Option<u16>,
    
    /// Blockchain file location
    #[arg(short, long, env = "BLOCKCHAIN_FILE")]
    blockchain_file: Option<String>,
    
    /// Addresses of initial peer nodes
    #[arg(short = 'n', long = "node", env = "INITIAL_PEERS", value_delimiter = ',')]
    nodes: Vec<String>,
    
    /// Path to configuration file
    #[arg(short, long, env = "CONFIG_FILE", default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments (includes environment variables)
    let args = Args::parse();
    
    // Load configuration from JSON file
    let config = BlockchainConfig::load_from_file(&args.config);
    
    // Priority: CLI args > Environment vars > JSON config > Defaults
    let port = args.port.unwrap_or(config.node.port);
    let blockchain_file = args.blockchain_file
        .unwrap_or_else(|| config.node.blockchain_file.clone());
    
    // Priority for nodes: CLI/env nodes if provided, otherwise config nodes
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
