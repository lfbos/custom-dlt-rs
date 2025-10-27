#[cfg(test)]
mod transaction_tests {
    use crate::types::{Transaction, TransactionInput, TransactionOutput};
    use crate::crypto::PrivateKey;
    use crate::sha256::Hash;
    use uuid::Uuid;

    fn create_test_output(value: u64, private_key: &mut PrivateKey) -> TransactionOutput {
        TransactionOutput {
            value,
            unique_id: Uuid::new_v4(),
            pubkey: private_key.public_key(),
        }
    }

    fn create_test_input(output_hash: &Hash, private_key: &mut PrivateKey) -> TransactionInput {
        use crate::crypto::Signature;
        TransactionInput {
            prev_transaction_output_hash: *output_hash,
            signature: Signature::sign_output(output_hash, private_key),
        }
    }

    #[test]
    fn test_transaction_creation() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let output_hash = output.hash();

        let transaction = Transaction::new(
            vec![],
            vec![output.clone()],
        );

        assert_eq!(transaction.outputs.len(), 1);
        assert_eq!(transaction.outputs[0].value, 1000);
        assert_eq!(transaction.outputs[0].hash(), output_hash);
    }

    #[test]
    fn test_transaction_hashing() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);

        let tx = Transaction::new(vec![], vec![output.clone()]);

        // Same transaction should produce same hash
        assert_eq!(tx.hash(), tx.hash());
    }
    
    #[test]
    fn test_transaction_different_hashes() {
        let mut private_key = PrivateKey::new_key();
        let output1 = create_test_output(1000, &mut private_key);
        let output2 = create_test_output(1000, &mut private_key);

        let tx1 = Transaction::new(vec![], vec![output1]);
        let tx2 = Transaction::new(vec![], vec![output2]);

        // Different transactions should produce different hashes
        // (due to unique IDs in outputs)
        assert_ne!(tx1.hash(), tx2.hash());
    }

    #[test]
    fn test_transaction_output_hashing() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);

        let hash1 = output.hash();
        let hash2 = output.hash();

        // Same output should always produce same hash
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_transaction_output_value() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(5000, &mut private_key);

        assert_eq!(output.value, 5000);
    }

    #[test]
    fn test_transaction_with_inputs() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let output_hash = output.hash();

        let input = create_test_input(&output_hash, &mut private_key);

        let transaction = Transaction::new(
            vec![input],
            vec![create_test_output(800, &mut private_key)],
        );

        assert_eq!(transaction.inputs.len(), 1);
        assert_eq!(transaction.outputs.len(), 1);
    }
}

#[cfg(test)]
mod block_tests {
    use crate::types::{Block, BlockHeader, Transaction, TransactionOutput};
    use crate::crypto::PrivateKey;
    use crate::util::MerkleRoot;
    use chrono::Utc;
    use crate::config;
    use uuid::Uuid;
    
    fn create_test_output(value: u64, private_key: &mut PrivateKey) -> TransactionOutput {
        TransactionOutput {
            value,
            unique_id: Uuid::new_v4(),
            pubkey: private_key.public_key(),
        }
    }

    #[test]
    fn test_block_creation() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);

        let block = Block::new(
            BlockHeader::new(
                Utc::now(),
                0,
                crate::sha256::Hash::zero(),
                MerkleRoot::calculate(&vec![transaction.clone()]),
                config::min_target(),
            ),
            vec![transaction],
        );

        assert_eq!(block.transactions.len(), 1);
    }

    #[test]
    fn test_block_hashing() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);

        let block = Block::new(
            BlockHeader::new(
                Utc::now(),
                0,
                crate::sha256::Hash::zero(),
                MerkleRoot::calculate(&vec![transaction.clone()]),
                config::min_target(),
            ),
            vec![transaction],
        );

        // Same block should produce same hash
        assert_eq!(block.hash(), block.hash());
    }

    #[test]
    fn test_block_header_hash() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);

        let header = BlockHeader::new(
            Utc::now(),
            42,
            crate::sha256::Hash::zero(),
            MerkleRoot::calculate(&vec![transaction.clone()]),
            config::min_target(),
        );

        // Header hash should not be zero
        let hash = header.hash();
        assert_ne!(hash, crate::sha256::Hash::zero());
    }

    #[test]
    fn test_block_header_different_nonces_different_hash() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(1000, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);

        let header1 = BlockHeader::new(
            Utc::now(),
            0,
            crate::sha256::Hash::zero(),
            MerkleRoot::calculate(&vec![transaction.clone()]),
            config::min_target(),
        );

        let header2 = BlockHeader::new(
            Utc::now(),
            1,  // Different nonce
            crate::sha256::Hash::zero(),
            MerkleRoot::calculate(&vec![transaction.clone()]),
            config::min_target(),
        );

        // Different nonces should produce different hashes
        assert_ne!(header1.hash(), header2.hash());
    }
}

#[cfg(test)]
mod blockchain_tests {
    use crate::types::{Blockchain, Block, BlockHeader, Transaction, TransactionOutput};
    use crate::crypto::PrivateKey;
    use crate::util::MerkleRoot;
    use crate::{config, U256};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_output(value: u64, private_key: &mut PrivateKey) -> TransactionOutput {
        TransactionOutput {
            value,
            unique_id: Uuid::new_v4(),
            pubkey: private_key.public_key(),
        }
    }

    #[test]
    fn test_blockchain_initialization() {
        let blockchain = Blockchain::new();
        
        assert_eq!(blockchain.blocks().count(), 0);
        assert_eq!(blockchain.utxos().len(), 0);
        assert_eq!(blockchain.block_height(), 0);
    }

    #[test]
    fn test_blockchain_add_genesis_block() {
        let mut blockchain = Blockchain::new();
        let mut private_key = PrivateKey::new_key();
        
        let output = create_test_output(config::initial_reward() * 100_000_000, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);
        
        let block = Block::new(
            BlockHeader::new(
                Utc::now(),
                0,
                crate::sha256::Hash::zero(),
                MerkleRoot::calculate(&vec![transaction.clone()]),
                config::min_target(),
            ),
            vec![transaction],
        );

        let result = blockchain.add_block(block);
        assert!(result.is_ok());
        assert_eq!(blockchain.block_height(), 1);
    }

    #[test]
    fn test_calculate_block_reward() {
        let blockchain = Blockchain::new();
        
        // At height 0, reward should be initial_reward
        blockchain.calculate_block_reward();
        assert_eq!(blockchain.block_height(), 0);
        
        // Test that reward calculation exists
        let reward = blockchain.calculate_block_reward();
        assert!(reward > 0);
    }

    #[test]
    fn test_blockchain_target() {
        let blockchain = Blockchain::new();
        let target = blockchain.target();
        
        // Target should not be zero
        assert_ne!(target, U256::from(0));
    }
}

