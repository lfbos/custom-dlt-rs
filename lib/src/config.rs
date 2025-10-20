/// Configuration module for blockchain parameters
///
/// This module provides a centralized configuration system that supports:
/// - JSON configuration files (primary method)
/// - Multiple network profiles (mainnet, testnet, devnet)
/// - Hardcoded defaults (fallback)
///
/// Configuration priority:
/// 1. JSON config file (config.json)
/// 2. Hardcoded defaults (fallback)

use crate::U256;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;

/// Default configuration file name
pub const DEFAULT_CONFIG_FILE: &str = "config.json";

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
            initial_reward: crate::INITIAL_REWARD,
            halving_interval: crate::HALVING_INTERVAL,
            ideal_block_time: crate::IDEAL_BLOCK_TIME,
            difficulty_update_interval: crate::DIFFICULTY_UPDATE_INTERVAL,
            max_mempool_transaction_age: crate::MAX_MEMPOOL_TRANSACTION_AGE,
            block_transaction_cap: crate::BLOCK_TRANSACTION_CAP,
            // Convert U256 constant to hex string
            min_target_hex: format!("0x{:x}", crate::MIN_TARGET),
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
    /// Load configuration from JSON file or use defaults
    /// 
    /// Configuration priority:
    /// 1. JSON config file (config.json)
    /// 2. Hardcoded defaults (fallback)
    pub fn load() -> Self {
        Self::load_from_file(DEFAULT_CONFIG_FILE)
    }
    
    /// Load configuration from a specific file path
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        
        // Try to load JSON config file
        if path.exists() {
            match std::fs::read_to_string(path) {
                Ok(contents) => match serde_json::from_str::<BlockchainConfig>(&contents) {
                    Ok(cfg) => {
                        eprintln!("✓ Loaded configuration from {}", path.display());
                        return cfg;
                    }
                    Err(e) => {
                        eprintln!("⚠ Warning: Failed to parse {}: {}", path.display(), e);
                        eprintln!("  Using defaults instead");
                    }
                },
                Err(e) => {
                    eprintln!("⚠ Warning: Could not read {}: {}", path.display(), e);
                    eprintln!("  Using defaults instead");
                }
            }
        } else {
            eprintln!("ℹ No config file found at {}, using defaults", path.display());
        }
        
        // Fallback to defaults
        BlockchainConfig::default()
    }
    
    /// Save configuration to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path.as_ref(), json)?;
        Ok(())
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
                crate::MIN_TARGET
            })
    }
}

// =============================================================================
// Helper Functions for Easy Access
// =============================================================================
// These functions provide easy access to configuration values from the global config.

/// Get initial reward from config
pub fn initial_reward() -> u64 {
    BlockchainConfig::global().network.initial_reward
}

/// Get halving interval from config
pub fn halving_interval() -> u64 {
    BlockchainConfig::global().network.halving_interval
}

/// Get ideal block time from config
pub fn ideal_block_time() -> u64 {
    BlockchainConfig::global().network.ideal_block_time
}

/// Get minimum target from config
pub fn min_target() -> U256 {
    BlockchainConfig::global().min_target()
}

/// Get difficulty update interval from config
pub fn difficulty_update_interval() -> u64 {
    BlockchainConfig::global().network.difficulty_update_interval
}

/// Get max mempool transaction age from config
pub fn max_mempool_transaction_age() -> u64 {
    BlockchainConfig::global().network.max_mempool_transaction_age
}

/// Get block transaction cap from config
pub fn block_transaction_cap() -> usize {
    BlockchainConfig::global().network.block_transaction_cap
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

