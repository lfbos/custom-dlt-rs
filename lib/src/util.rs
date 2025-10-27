use crate::sha256::Hash;
use crate::types::Transaction;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Result as IoResult, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MerkleRoot(Hash);

impl MerkleRoot {
    /// Calculates the Merkle root of a block's transactions.
    ///
    /// A Merkle tree is a binary tree where:
    /// - Leaf nodes are hashes of individual transactions
    /// - Non-leaf nodes are hashes of their children
    /// - The root is a single hash committing to all transactions
    ///
    /// # Example Structure:
    ///
    /// ```text
    /// For 4 transactions [A, B, C, D]:
    ///
    ///          ROOT = H(H(AB) || H(CD))
    ///          /                      \
    ///      H(AB)                    H(CD)
    ///      /    \                   /    \
    ///   H(A)   H(B)             H(C)   H(D)
    ///     |      |                |      |
    ///     A      B                C      D
    /// ```
    ///
    /// # Odd Number of Transactions:
    ///
    /// If there's an odd number at any level, the last hash is duplicated:
    ///
    /// ```text
    /// For 3 transactions [A, B, C]:
    ///
    ///          ROOT = H(H(AB) || H(CC))
    ///          /                      \
    ///      H(AB)                    H(CC)   ← C is duplicated
    ///      /    \                   /    \
    ///   H(A)   H(B)             H(C)   H(C)
    /// ```
    ///
    /// # Benefits:
    /// - **Efficient verification**: Can prove a transaction is in block with O(log n) hashes
    /// - **Tamper evidence**: Changing any transaction changes the root
    /// - **Light clients**: Don't need all transactions to verify inclusion
    pub fn calculate(transactions: &[Transaction]) -> MerkleRoot {
        // STEP 1: Create the bottom layer (leaf nodes)
        // =============================================
        // Hash each transaction to form the leaves of the tree
        let mut layer: Vec<Hash> = vec![];
        for transaction in transactions {
            layer.push(Hash::hash(transaction));
        }

        // STEP 2: Build tree bottom-up until we have a single root
        // =========================================================
        // Keep combining pairs of hashes until only one remains
        while layer.len() > 1 {
            let mut new_layer = vec![];

            // Process hashes in pairs
            for pair in layer.chunks(2) {
                let left = pair[0];

                // If there's an odd number, duplicate the last hash
                // Example: [A, B, C] becomes pairs [(A,B), (C,C)]
                let right = pair.get(1).unwrap_or(&pair[0]);

                // Combine the pair by hashing them together
                // H(left || right) where || means concatenation
                new_layer.push(Hash::hash(&[left, *right]));
            }

            // Move up one level in the tree
            layer = new_layer;
        }

        // STEP 3: Return the root (single remaining hash)
        // ===============================================
        MerkleRoot(layer[0])
    }
}

pub trait Saveable
where
    Self: Sized,
{
    fn load<I: Read>(reader: I) -> IoResult<Self>;
    fn save<O: Write>(&self, writer: O) -> IoResult<()>;
    fn save_to_file<P: AsRef<Path>>(&self, path: P) -> IoResult<()> {
        let file = File::create(&path)?;
        self.save(file)
    }
    fn load_from_file<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        let file = File::open(&path)?;
        Self::load(file)
    }
}

#[cfg(test)]
mod tests;
