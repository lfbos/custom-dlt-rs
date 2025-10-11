use std::{env, process::exit, vec};

use btclib::{
    U256,
    crypto::PrivateKey,
    sha256::Hash,
    types::{Block, BlockHeader, Transaction, TransactionOutput},
    util::{MerkleRoot, Saveable},
};
use chrono::Utc;
use uuid::Uuid;

fn main() {
    let path = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        eprintln!("Usage: block_gen <block_file> [custom_target_hex]");
        eprintln!("  custom_target_hex: optional hex string to set custom difficulty");
        eprintln!(
            "  Example: block_gen myblock.cbor 0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        );
        exit(1);
    };

    // Check for optional custom target argument
    let target = if let Some(target_hex) = env::args().nth(2) {
        // Parse hex string into U256
        match U256::from_str_radix(&target_hex, 16) {
            Ok(t) => {
                println!("Using custom target: {:#x}", t);
                t
            }
            Err(_) => {
                eprintln!("Error: Invalid hex string for custom target");
                eprintln!("Expected format: 64 hex characters (0-9, a-f)");
                exit(1);
            }
        }
    } else {
        println!("Using MIN_TARGET: {:#x}", btclib::MIN_TARGET);
        btclib::MIN_TARGET
    };

    let private_key = PrivateKey::new_key();
    let transactions = vec![Transaction::new(
        vec![],
        vec![TransactionOutput {
            unique_id: Uuid::new_v4(),
            value: btclib::INITIAL_REWARD * 10u64.pow(8),
            pubkey: private_key.public_key(),
        }],
    )];
    let merkle_root = MerkleRoot::calculate(&transactions);
    let block = Block::new(
        BlockHeader::new(Utc::now(), 0, Hash::zero(), merkle_root, target),
        transactions,
    );
    block.save_to_file(path).expect("Failed to save block");
    println!("Block generated successfully!");
}
