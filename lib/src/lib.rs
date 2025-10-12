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
// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds
pub const IDEAL_BLOCK_TIME: u64 = 10;
// minimum target
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
// difficulty update interval in blocks
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
// maximum mempool transaction age in seconds
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;
// maximum amount of transactions allowed in a block
pub const BLOCK_TRANSACTION_CAP: usize = 20;
pub mod crypto;
pub mod error;
pub mod network;
pub mod sha256;
pub mod util;

#[path = "../types/mod.rs"]
pub mod types;
