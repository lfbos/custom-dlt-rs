/// Configuration module for blockchain parameters
///
/// This module provides a centralized configuration system that supports:
/// - Hardcoded defaults (for educational simplicity)
/// - Environment variable overrides (for flexibility)
/// - Multiple network profiles (mainnet, testnet, devnet)
///
/// Configuration priority (highest to lowest):
/// 1. Environment variables
/// 2. .env file
/// 3. Hardcoded defaults

use crate::U256;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Global configuration instance
static CONFIG: OnceLock<BlockchainConfig> = OnceLock::new();

/// Complete blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Network configuration (consensus rules)
    pub network: NetworkConfig,
    
    /// Node-specific settings
    pub node: NodeConfig,
    
    /// Mining parameters
    pub mining: MiningConfig,
    
    /// Wallet settings
    pub wallet: WalletConfig,
}

/// Network consensus parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Network identifier (mainnet, testnet, devnet)
    pub network_id: String,
    
    /// Initial block reward in whole coins (multiplied by 10^8 for satoshis)
    pub initial_reward: u64,
    
    /// Number of blocks between reward halvings
    pub halving_interval: u64,
    
    /// Target time between blocks in seconds
    pub ideal_block_time: u64,
    
    /// Number of blocks between difficulty adjustments
    pub difficulty_update_interval: u64,
    
    /// Maximum age of mempool transactions in seconds
    pub max_mempool_transaction_age: u64,
    
    /// Maximum number of transactions per block
    pub block_transaction_cap: usize,
    
    /// Minimum difficulty target (easiest difficulty)
    /// Format: hex string like "0x0000FFFFFFFFFFFF..."
    pub min_target_hex: String,
}

/// Node operation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Port to listen on
    pub port: u16,
    
    /// Blockchain file path
    pub blockchain_file: String,
    
    /// Initial peer addresses (comma-separated)
    pub initial_peers: Vec<String>,
    
    /// Mempool cleanup interval in seconds
    pub mempool_cleanup_interval_secs: u64,
    
    /// Blockchain save interval in seconds
    pub blockchain_save_interval_secs: u64,
    
    /// Maximum number of peer connections
    pub max_peers: usize,
}

/// Mining configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    /// Number of nonces to try in each mining batch
    pub mining_batch_size: usize,
    
    /// Seconds between template fetches/validations
    pub template_fetch_interval_secs: u64,
    
    /// Node address to connect to
    pub node_address: String,
    
    /// Public key file for receiving rewards
    pub public_key_file: String,
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// UTXO update interval in seconds
    pub utxo_update_interval_secs: u64,
    
    /// Balance display update interval in milliseconds
    pub balance_display_update_interval_ms: u64,
    
    /// Node address to connect to
    pub node_address: String,
    
    /// Wallet configuration file path
    pub config_file: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            network_id: "mainnet".to_string(),
            initial_reward: 50,
            halving_interval: 210,
            ideal_block_time: 10,
            difficulty_update_interval: 50,
            max_mempool_transaction_age: 600,
            block_transaction_cap: 20,
            min_target_hex: "0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_string(),
        }
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            port: 9000,
            blockchain_file: "./blockchain.cbor".to_string(),
            initial_peers: vec![],
            mempool_cleanup_interval_secs: 30,
            blockchain_save_interval_secs: 15,
            max_peers: 50,
        }
    }
}

impl Default for MiningConfig {
    fn default() -> Self {
        Self {
            mining_batch_size: 2_000_000,
            template_fetch_interval_secs: 5,
            node_address: "127.0.0.1:9000".to_string(),
            public_key_file: "miner.pub.pem".to_string(),
        }
    }
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            utxo_update_interval_secs: 20,
            balance_display_update_interval_ms: 500,
            node_address: "127.0.0.1:9000".to_string(),
            config_file: "wallet_config.toml".to_string(),
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            node: NodeConfig::default(),
            mining: MiningConfig::default(),
            wallet: WalletConfig::default(),
        }
    }
}

impl BlockchainConfig {
    /// Load configuration with the following priority:
    /// 1. Environment variables (highest priority)
    /// 2. .env file
    /// 3. Hardcoded defaults (lowest priority)
    pub fn load() -> Self {
        // Try to load .env file (fails silently if not found)
        dotenvy::dotenv().ok();
        
        let mut config = BlockchainConfig::default();
        
        // Override with environment variables
        config.network = NetworkConfig::from_env();
        config.node = NodeConfig::from_env();
        config.mining = MiningConfig::from_env();
        config.wallet = WalletConfig::from_env();
        
        config
    }
    
    /// Get or initialize the global configuration
    pub fn global() -> &'static BlockchainConfig {
        CONFIG.get_or_init(|| BlockchainConfig::load())
    }
    
    /// Parse MIN_TARGET from hex string
    pub fn min_target(&self) -> U256 {
        let hex_str = self.network.min_target_hex.trim_start_matches("0x");
        U256::from_str_radix(hex_str, 16)
            .unwrap_or_else(|_| {
                eprintln!("Warning: Invalid MIN_TARGET_HEX, using default");
                U256([
                    0xFFFF_FFFF_FFFF_FFFF,
                    0xFFFF_FFFF_FFFF_FFFF,
                    0xFFFF_FFFF_FFFF_FFFF,
                    0x0000_FFFF_FFFF_FFFF,
                ])
            })
    }
}

impl NetworkConfig {
    fn from_env() -> Self {
        Self {
            network_id: env_var("NETWORK_ID").unwrap_or_else(|| "mainnet".to_string()),
            initial_reward: parse_env("INITIAL_REWARD").unwrap_or(50),
            halving_interval: parse_env("HALVING_INTERVAL").unwrap_or(210),
            ideal_block_time: parse_env("IDEAL_BLOCK_TIME").unwrap_or(10),
            difficulty_update_interval: parse_env("DIFFICULTY_UPDATE_INTERVAL").unwrap_or(50),
            max_mempool_transaction_age: parse_env("MAX_MEMPOOL_TX_AGE").unwrap_or(600),
            block_transaction_cap: parse_env("BLOCK_TX_CAP").unwrap_or(20),
            min_target_hex: env_var("MIN_TARGET_HEX")
                .unwrap_or_else(|| "0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF".to_string()),
        }
    }
}

impl NodeConfig {
    fn from_env() -> Self {
        let initial_peers_str = env_var("INITIAL_PEERS").unwrap_or_default();
        let initial_peers = if initial_peers_str.is_empty() {
            vec![]
        } else {
            initial_peers_str.split(',').map(|s| s.trim().to_string()).collect()
        };
        
        Self {
            port: parse_env("NODE_PORT").unwrap_or(9000),
            blockchain_file: env_var("BLOCKCHAIN_FILE").unwrap_or_else(|| "./blockchain.cbor".to_string()),
            initial_peers,
            mempool_cleanup_interval_secs: parse_env("MEMPOOL_CLEANUP_INTERVAL").unwrap_or(30),
            blockchain_save_interval_secs: parse_env("BLOCKCHAIN_SAVE_INTERVAL").unwrap_or(15),
            max_peers: parse_env("MAX_PEERS").unwrap_or(50),
        }
    }
}

impl MiningConfig {
    fn from_env() -> Self {
        Self {
            mining_batch_size: parse_env("MINING_BATCH_SIZE").unwrap_or(2_000_000),
            template_fetch_interval_secs: parse_env("TEMPLATE_FETCH_INTERVAL").unwrap_or(5),
            node_address: env_var("MINER_NODE_ADDRESS").unwrap_or_else(|| "127.0.0.1:9000".to_string()),
            public_key_file: env_var("MINER_PUBLIC_KEY").unwrap_or_else(|| "miner.pub.pem".to_string()),
        }
    }
}

impl WalletConfig {
    fn from_env() -> Self {
        Self {
            utxo_update_interval_secs: parse_env("UTXO_UPDATE_INTERVAL").unwrap_or(20),
            balance_display_update_interval_ms: parse_env("BALANCE_UPDATE_INTERVAL_MS").unwrap_or(500),
            node_address: env_var("WALLET_NODE_ADDRESS").unwrap_or_else(|| "127.0.0.1:9000".to_string()),
            config_file: env_var("WALLET_CONFIG_FILE").unwrap_or_else(|| "wallet_config.toml".to_string()),
        }
    }
}

/// Helper function to get environment variable
fn env_var(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

/// Helper function to parse environment variable
fn parse_env<T: std::str::FromStr>(key: &str) -> Option<T> {
    env_var(key)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config_loads() {
        let config = BlockchainConfig::default();
        assert_eq!(config.network.initial_reward, 50);
        assert_eq!(config.node.port, 9000);
    }
    
    #[test]
    fn test_min_target_parsing() {
        let config = BlockchainConfig::default();
        let target = config.min_target();
        assert!(target > U256::zero());
    }
}

