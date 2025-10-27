//! Integration Tests for the Blockchain System
//!
//! ## 🎓 For New Team Members: Key Concepts
//!
//! If you're new to blockchain, here are the key terms you need to understand:
//!
//! ### **Blockchain Basics**
//!
//! **What is a blockchain?**
//! - A chain of linked blocks, each containing transactions
//! - Like a ledger that everyone agrees on
//! - Immutable: you can't change past blocks
//!
//! **What is a Genesis Block?**
//! - The **very first block** in the blockchain (Block #0)
//! - Special: has no "previous block" (prev_hash = all zeros)
//! - Creates the initial money supply
//! - Only ONE genesis block exists per blockchain
//!
//! **What is a Coinbase Transaction?**
//! - The **first transaction in every block**
//! - Creates new coins (mining reward)
//! - Has NO inputs (creates money from nothing)
//! - Rewards the miner who found the block
//!
//! **What is a UTXO?** (Unspent Transaction Output)
//! - Think of it as a "coin" or "bill"
//! - When someone sends you money, you receive a UTXO
//! - When you spend money, you consume UTXOs
//! - Like cash: you spend whole bills, get change back
//!
//! **What is the Mempool?**
//! - A waiting room for transactions
//! - New transactions wait here before being included in blocks
//! - Miners pick transactions from the mempool to include in blocks
//! - Think: like a post office sorting mail before delivery
//!
//! **What is Proof of Work (PoW)?**
//! - Miners solve a cryptographic puzzle to create a block
//! - The puzzle: "find a number (nonce) that makes the block hash start with zeros"
//! - The more zeros required, the harder the puzzle
//! - First miner to solve it gets the block reward
//!
//! ### Test Strategy
//!
//! - **In-Memory Operations**: Tests run in memory without actual network I/O
//! - **Full Workflow Verification**: Tests complete blockchain workflows
//! - **State Consistency**: Verifies blockchain state is maintained correctly
//!
//! ### Integration Test Scenarios
//!
//! **Test #1: Genesis Block Creation**
//! - Tests that the blockchain can start from scratch
//! - Verifies the genesis block is created correctly
//! - Ensures initial UTXOs are created
//!
//! **Test #2: Transaction to Mempool**
//! - Tests that transactions can wait in the mempool
//! - Verifies UTXO references are valid
//! - Ensures transactions are properly queued
//!
//! **Test #3: Multiple Blocks**
//! - Tests that blocks can be mined and added
//! - Verifies Proof of Work (nonce finding)
//! - Ensures blockchain grows correctly
//!
//! **Test #4: State Consistency**
//! - Tests that blockchain state is maintained correctly
//! - Verifies UTXOs are tracked properly
//! - Ensures block height is accurate
//!
//! ## Test Architecture
//!
//! All tests use in-memory blockchain instances (no real network/files) to make tests:
//! - Fast (milliseconds, not seconds)
//! - Reliable (no network issues)
//! - Easy to debug
//! - Suitable for CI/CD
//!
//! ## 🚧 What We're NOT Testing Here (And Why)
//!
//! These integration tests focus on **core blockchain logic** (Blockchain struct in isolation).
//! We do NOT test:
//!
//! - **Node + Miner interaction** (miner submitting blocks to node)
//! - **Node + Wallet interaction** (wallet sending transactions to node)
//! - **Full end-to-end workflows** (wallet -> node -> miner -> blockchain)
//!
//! **Why?** The node, miner, and wallet components are:
//! - Network-dependent (require TCP streams)
//! - Async-heavy (Tokio runtime)
//! - State-dependent (global shared state)
//! - Integration-tested via the Docker Compose setup
//!
//! **Real-World Testing:**
//! - Run the full system: `docker-compose up`
//! - This tests node + miner + wallet working together
//! - See `docker/README.md` for instructions

use btclib::crypto::PrivateKey;
use btclib::types::{Block, BlockHeader, Blockchain, Transaction, TransactionOutput};
use btclib::util::MerkleRoot;
use btclib::config;
use chrono::Utc;
use btclib::sha256::Hash;
use uuid::Uuid;

/// Helper function to create a test transaction output
///
/// # Parameters
/// - `value`: Amount in satoshis
/// - `private_key`: Private key to derive public key from
///
/// # Returns
/// A `TransactionOutput` ready to be included in a transaction
fn create_test_output(value: u64, private_key: &mut PrivateKey) -> TransactionOutput {
    TransactionOutput {
        value,
        unique_id: Uuid::new_v4(),
        pubkey: private_key.public_key(),
    }
}

/// Helper: Create a blockchain with a genesis block containing UTXOs
///
/// This helper function sets up a blockchain from scratch by:
/// 1. Creating a new empty blockchain
/// 2. Generating a miner's keypair
/// 3. Creating a genesis block with a coinbase transaction
/// 4. Adding the genesis block to the blockchain
/// 5. Rebuilding the UTXO set
///
/// # Returns
/// A tuple of (blockchain instance, miner private key) ready for testing
fn create_blockchain_with_genesis(_initial_balance: u64) -> (Blockchain, PrivateKey) {
    let mut blockchain = Blockchain::new();
    let mut miner_key = PrivateKey::new_key();
    
    // Get the initial reward from config
    let reward = config::initial_reward() * 100_000_000;
    
    // Create genesis block with a proper coinbase transaction
    // For genesis, we create a coinbase transaction (no inputs) that creates UTXOs
    let genesis_output = create_test_output(reward, &mut miner_key);
    let genesis_tx = Transaction::new(vec![], vec![genesis_output]);
    
    let genesis_block = Block::new(
        BlockHeader::new(
            Utc::now(),
            0, // nonce
            Hash::zero(), // prev_block_hash must be zero for genesis
            MerkleRoot::calculate(&vec![genesis_tx.clone()]), // merkle_root
            config::min_target(), // target
        ),
        vec![genesis_tx],
    );
    
    // Add the block
    let result = blockchain.add_block(genesis_block);
    match result {
        Ok(_) => {},
        Err(e) => panic!("Failed to add genesis block: {:?}", e),
    }
    
    // Rebuild UTXOs after adding block
    blockchain.rebuild_utxos();
    
    (blockchain, miner_key)
}

/// Test #1: Genesis Block Creation
///
/// **What it tests:** Can we start a blockchain from scratch?
///
/// **For beginners:** Every blockchain needs a starting point. The genesis block is Block #0,
/// and it's special because it has no "previous block". This test creates one and verifies
/// everything is set up correctly.
///
/// **What happens:**
/// 1. Create an empty blockchain
/// 2. Add a genesis block with a coinbase transaction (creates initial coins)
/// 3. Rebuild the UTXO set (track available coins)
///
/// **What we verify:**
/// - Blockchain has 1 block (the genesis block)
/// - We have 1 UTXO (the initial coins created by the coinbase transaction)
#[test]
fn test_genesis_block_creation() {
    let (blockchain, _) = create_blockchain_with_genesis(1000);
    
    // Genesis block should have 1 block
    assert_eq!(blockchain.block_height(), 1);
    
    // Genesis block should have UTXOs
    assert_eq!(blockchain.utxos().len(), 1);
}

/// Test #2: Transaction to Mempool
///
/// **What it tests:** Can someone create a transaction and have it wait in the mempool?
///
/// **For beginners:** When Alice wants to send Bob some money:
/// 1. Alice creates a transaction ("give Bob 500 coins from my UTXO")
/// 2. The transaction waits in the mempool (waiting room)
/// 3. A miner picks it up and includes it in a block
/// 4. The block is added to the blockchain
///
/// This test simulates step 1-2: creating a valid transaction and adding it to the mempool.
///
/// **What happens:**
/// 1. Create a blockchain with genesis block (has 1 UTXO)
/// 2. Create a transaction that spends that UTXO
/// 3. Add the transaction to the mempool
///
/// **What we verify:**
/// - Transaction has valid references to UTXOs
/// - Transaction is accepted by the mempool
/// - Mempool has 1 pending transaction
#[test]
fn test_add_transaction_to_mempool() {
    // Start with a fresh blockchain (has genesis block with UTXOs)
    let (mut blockchain, miner_key) = create_blockchain_with_genesis(1000);
    
    // Get the first available UTXO from the genesis block
    let utxo_hash = blockchain.utxos().keys().next().unwrap().clone();
    
    // Create a valid transaction that spends the UTXO
    let recipient_key = PrivateKey::new_key();
    let mut miner_key_copy = miner_key; // Copy for signing
    let tx_input = btclib::types::TransactionInput {
        prev_transaction_output_hash: utxo_hash,
        signature: btclib::crypto::Signature::sign_output(&utxo_hash, &mut miner_key_copy),
    };
    
    let mut recipient_key_copy = recipient_key;
    let tx_output = create_test_output(500, &mut recipient_key_copy);
    
    let transaction = Transaction::new(vec![tx_input], vec![tx_output]);
    
    // Add to mempool - should succeed if UTXO is valid
    let result = blockchain.add_to_mempool(transaction);
    assert!(result.is_ok(), "Transaction should be added to mempool");
    
    // Verify mempool has 1 transaction
    assert_eq!(blockchain.mempool().len(), 1);
}

/// Test #3: Multiple Blocks
///
/// **What it tests:** Can we mine blocks and add them to the blockchain?
///
/// **For beginners:** In a real blockchain, miners compete to add blocks. They:
/// 1. Collect transactions from the mempool
/// 2. Create a block
/// 3. Solve a cryptographic puzzle (Proof of Work)
/// 4. Add the block to the blockchain
///
/// This test simulates a miner creating a second block after the genesis block.
///
/// **What happens:**
/// 1. Start with a genesis block
/// 2. Create a second block with a coinbase transaction
/// 3. Mine the block (find a valid nonce)
/// 4. Add the block to the blockchain
///
/// **What we verify:**
/// - Block is properly mined (hash meets the target)
/// - Block is accepted and added
/// - Blockchain has 2 blocks now
#[test]
fn test_multiple_blocks() {
    let (mut blockchain, _) = create_blockchain_with_genesis(1000);
    
    // Verify genesis
    assert_eq!(blockchain.block_height(), 1);
    assert_eq!(blockchain.utxos().len(), 1);
    
    // Add a second block with a coinbase transaction
    // Note: Every block has a coinbase transaction as the first transaction
    // For integration tests, we're verifying the blockchain structure works
    // Full transaction validation is already tested in unit tests
    let prev_hash = blockchain.blocks().last().unwrap().hash();
    let mut new_miner_key = PrivateKey::new_key();
    
    // Use same reward as genesis for simplicity
    let block_reward = config::initial_reward() * 100_000_000;
    let coinbase_output = create_test_output(block_reward, &mut new_miner_key);
    let coinbase_tx = Transaction::new(vec![], vec![coinbase_output]);
    
    let mut block = Block::new(
        BlockHeader::new(
            Utc::now() + chrono::Duration::seconds(1),
            0,
            prev_hash,
            MerkleRoot::calculate(&vec![coinbase_tx.clone()]),
            config::min_target(),
        ),
        vec![coinbase_tx],
    );
    
    // Mine the block (Proof of Work)
    // Try different nonces until we find one that makes the hash meet the target
    if !block.header.hash().matches_target(block.header.target) {
        for nonce in 0..=1_000_000 {
            block.header.nonce = nonce;
            if block.header.hash().matches_target(block.header.target) {
                break;
            }
        }
    }
    
    // Add block
    blockchain.add_block(block).expect("Block should be valid");
    blockchain.rebuild_utxos();
    
    // Verify: Block added
    assert_eq!(blockchain.block_height(), 2);
}

/// Test #4: Blockchain State Consistency
///
/// **What it tests:** Does the blockchain keep track of everything correctly?
///
/// **For beginners:** A blockchain is like a database. It needs to:
/// - Track how many blocks exist
/// - Track how many unspent coins (UTXOs) exist
/// - Keep this information consistent
///
/// This test checks that after creating a genesis block, our blockchain knows:
/// - "I have 1 block"
/// - "I have 1 UTXO (the coins created by the coinbase)"
///
/// **What happens:**
/// 1. Create a genesis block
/// 2. Check blockchain state
///
/// **What we verify:**
/// - Blockchain height is correct (1 block)
/// - UTXO count is correct (1 UTXO from genesis)
/// - State is internally consistent
#[test]
fn test_blockchain_state_consistency() {
    let (blockchain, _) = create_blockchain_with_genesis(1000);
    
    // Initial state after genesis
    assert_eq!(blockchain.block_height(), 1);
    assert_eq!(blockchain.utxos().len(), 1);
    
    // UTXOs should be present from genesis
    assert!(blockchain.utxos().len() > 0);
}

