//! # BtcLib - Educational Blockchain Library
//!
//! This library implements core blockchain functionality based on Bitcoin's design.
//! It is an educational implementation based on the book "Building Bitcoin in Rust".
//!
//! ## Attribution
//! - **Based on:** "Building Bitcoin in Rust" (book)
//! - **Implementation:** Luis Boscan (@lfbos)
//! - **License:** MIT
//! - **Purpose:** Educational - to help others learn blockchain technology
//!
//! For detailed credits and acknowledgments, see CREDITS.md in the repository root.

use serde::{Deserialize, Serialize};
use uint::construct_uint;
construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 x 64-bit words
    #[derive(Deserialize, Serialize)]
    pub struct U256(4);
}
// =============================================================================
// BLOCKCHAIN PARAMETERS - Default Values
// =============================================================================
// These constants define the default blockchain parameters.
// They are used by the config module when no JSON config file is provided.
//
// USAGE:
//   - Direct use: Still works but not recommended
//   - Via config: config::initial_reward() (reads from JSON config or these defaults)
//
// CONFIGURATION:
//   To customize, create a JSON config file:
//     cargo run --bin config_gen
//     cp config.default.json config.json
//     nano config.json  # Edit: "initial_reward": 100
//
// The config module loads from config.json or uses these as fallback.
// =============================================================================

/// Initial reward in bitcoin - multiply by 10^8 to get satoshis
/// **Default value** used when no config.json is provided
pub const INITIAL_REWARD: u64 = 50;

/// Halving interval in blocks
/// **Default value** used when no config.json is provided
pub const HALVING_INTERVAL: u64 = 210;

/// Ideal block time in seconds
/// **Default value** used when no config.json is provided
pub const IDEAL_BLOCK_TIME: u64 = 10;

/// Minimum target (easiest difficulty)
/// **Default value** used when no config.json is provided
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);

/// Difficulty update interval in blocks
/// **Default value** used when no config.json is provided
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;

/// Maximum mempool transaction age in seconds
/// **Default value** used when no config.json is provided
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;

/// Maximum amount of transactions allowed in a block
/// **Default value** used when no config.json is provided
pub const BLOCK_TRANSACTION_CAP: usize = 20;

pub mod config;
pub mod crypto;
pub mod error;
pub mod network;
pub mod sha256;
pub mod util;

#[cfg(test)]
pub mod test_helpers;

#[path = "../types/mod.rs"]
pub mod types;
