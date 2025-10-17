use anyhow::Result;
use clap::{Parser, Subcommand};
use cursive::views::TextContent;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, info};
mod core;
mod tasks;
mod ui;
mod util;
use core::Core;
use tasks::{handle_transactions, ui_task, update_balance, update_utxos};
use util::{big_mode_btc, generate_dummy_config, setup_panic_hook, setup_tracing};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Path to wallet configuration file
    #[arg(short, long, value_name = "FILE", env = "WALLET_CONFIG", default_value = "wallet_config.toml")]
    config: PathBuf,
    
    /// Node address to connect to
    #[arg(short, long, value_name = "ADDRESS", env = "WALLET_NODE_ADDRESS")]
    node: Option<String>,
    
    /// Path to blockchain configuration file
    #[arg(long, env = "CONFIG_FILE", default_value = "config.json")]
    blockchain_config: String,
}

#[derive(Subcommand)]
enum Commands {
    GenerateConfig {
        #[arg(short, long, value_name = "FILE", default_value_os_t = PathBuf::from("wallet_config.toml"))]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing()?;
    setup_panic_hook();
    info!("Starting wallet application");
    
    // Parse command line arguments (includes environment variables)
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::GenerateConfig { output }) => {
            debug!("Generating dummy config at: {:?}", output);
            return generate_dummy_config(output);
        }
        None => (),
    }
    
    // Load blockchain configuration from JSON file
    use btclib::config::BlockchainConfig;
    let _blockchain_config = BlockchainConfig::load_from_file(&cli.blockchain_config);
    
    // Load wallet configuration from TOML file
    info!("Loading wallet config from: {:?}", cli.config);
    let mut core = Core::load(cli.config.clone()).await?;
    
    // Priority: CLI args > Environment vars > Wallet config
    if let Some(node) = cli.node {
        info!("Overriding default node with: {}", node);
        core.config.default_node = node;
    }
    let (tx_sender, tx_receiver) = kanal::bounded(10);
    core.tx_sender = tx_sender;
    let core = Arc::new(core);
    info!("Starting background tasks");
    let balance_content = TextContent::new(big_mode_btc(&core));
    tokio::select! {
        _ = ui_task(core.clone(), balance_content.clone()).await => (),
        _ = update_utxos(core.clone()).await => (),
        _ = handle_transactions(tx_receiver.clone_async(), core.clone()).await => (),
        _ = update_balance(core.clone(), balance_content).await => (),
    }
    Ok(())
}
