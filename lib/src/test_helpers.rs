//! Shared test helper functions for unit tests
//!
//! This module provides reusable test utilities to avoid duplication
//! across test modules in the codebase.

use crate::crypto::{PrivateKey, Signature};
use crate::sha256::Hash;
use crate::types::{TransactionInput, TransactionOutput};
use uuid::Uuid;

/// Create a test transaction output
pub fn create_test_output(value: u64, private_key: &mut PrivateKey) -> TransactionOutput {
    TransactionOutput {
        value,
        unique_id: Uuid::new_v4(),
        pubkey: private_key.public_key(),
    }
}

/// Create a test transaction input with signature
pub fn create_test_input(output_hash: &Hash, private_key: &mut PrivateKey) -> TransactionInput {
    TransactionInput {
        prev_transaction_output_hash: *output_hash,
        signature: Signature::sign_output(output_hash, private_key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::PrivateKey;

    #[test]
    fn test_create_test_output() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(100, &mut private_key);

        assert_eq!(output.value, 100);
        assert_eq!(output.pubkey, private_key.public_key());
    }

    #[test]
    fn test_create_test_input() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(100, &mut private_key);
        let hash = output.hash();
        let input = create_test_input(&hash, &mut private_key);

        assert_eq!(input.prev_transaction_output_hash, hash);
        // Verify the signature is valid
        let is_valid = input.signature.verify(&hash, &private_key.public_key());
        assert!(is_valid);
    }
}
