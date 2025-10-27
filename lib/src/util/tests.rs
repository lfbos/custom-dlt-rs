#[cfg(test)]
mod tests {
    use crate::crypto::PrivateKey;
    use crate::sha256::Hash;
    use crate::test_helpers::create_test_output;
    use crate::types::Transaction;
    use crate::util::MerkleRoot;

    #[test]
    fn test_merkle_root_single_transaction() {
        let mut private_key = PrivateKey::new_key();
        let output = create_test_output(100, &mut private_key);
        let transaction = Transaction::new(vec![], vec![output]);
        let transactions = vec![transaction];

        let merkle_root = MerkleRoot::calculate(&transactions);

        // Single transaction: root should be hash of that transaction
        assert_eq!(merkle_root.0, Hash::hash(&transactions[0]));
    }

    #[test]
    fn test_merkle_root_two_transactions() {
        let mut private_key = PrivateKey::new_key();

        let output1 = create_test_output(100, &mut private_key);
        let output2 = create_test_output(200, &mut private_key);

        let tx1 = Transaction::new(vec![], vec![output1]);
        let tx2 = Transaction::new(vec![], vec![output2]);
        let transactions = vec![tx1, tx2];

        let merkle_root = MerkleRoot::calculate(&transactions);

        // Two transactions: root should be hash of [H(tx1), H(tx2)]
        let hash1 = Hash::hash(&transactions[0]);
        let hash2 = Hash::hash(&transactions[1]);
        let expected = Hash::hash(&[hash1, hash2]);

        assert_eq!(merkle_root.0, expected);
    }

    #[test]
    fn test_merkle_root_three_transactions() {
        // Tests the odd number duplication behavior
        let mut private_key = PrivateKey::new_key();

        let output1 = create_test_output(100, &mut private_key);
        let output2 = create_test_output(200, &mut private_key);
        let output3 = create_test_output(300, &mut private_key);

        let tx1 = Transaction::new(vec![], vec![output1]);
        let tx2 = Transaction::new(vec![], vec![output2]);
        let tx3 = Transaction::new(vec![], vec![output3]);
        let transactions = vec![tx1, tx2, tx3];

        let merkle_root = MerkleRoot::calculate(&transactions);

        // Three transactions: should duplicate the last one
        let hash1 = Hash::hash(&transactions[0]);
        let hash2 = Hash::hash(&transactions[1]);
        let hash3 = Hash::hash(&transactions[2]);

        // First level: [(H1, H2), (H3, H3)]
        let left = Hash::hash(&[hash1, hash2]);
        let right = Hash::hash(&[hash3, hash3]);

        // Second level: hash of the two results
        let expected = Hash::hash(&[left, right]);

        assert_eq!(merkle_root.0, expected);
    }

    #[test]
    fn test_merkle_root_four_transactions() {
        let mut private_key = PrivateKey::new_key();

        let outputs = vec![
            create_test_output(100, &mut private_key),
            create_test_output(200, &mut private_key),
            create_test_output(300, &mut private_key),
            create_test_output(400, &mut private_key),
        ];

        let transactions: Vec<Transaction> = outputs
            .into_iter()
            .map(|output| Transaction::new(vec![], vec![output]))
            .collect();

        let merkle_root = MerkleRoot::calculate(&transactions);

        // Should handle even number cleanly
        // Structure: H(H(H1,H2), H(H3,H4))
        let hash1 = Hash::hash(&transactions[0]);
        let hash2 = Hash::hash(&transactions[1]);
        let hash3 = Hash::hash(&transactions[2]);
        let hash4 = Hash::hash(&transactions[3]);

        let left = Hash::hash(&[hash1, hash2]);
        let right = Hash::hash(&[hash3, hash4]);
        let expected = Hash::hash(&[left, right]);

        assert_eq!(merkle_root.0, expected);
    }

    #[test]
    fn test_merkle_root_consistency() {
        // Same transactions should produce same root
        let mut private_key = PrivateKey::new_key();

        let output1 = create_test_output(100, &mut private_key);
        let output2 = create_test_output(200, &mut private_key);

        let tx1 = Transaction::new(vec![], vec![output1]);
        let tx2 = Transaction::new(vec![], vec![output2]);

        let transactions1 = vec![tx1.clone(), tx2.clone()];
        let transactions2 = vec![tx1, tx2];

        let root1 = MerkleRoot::calculate(&transactions1);
        let root2 = MerkleRoot::calculate(&transactions2);

        assert_eq!(root1, root2);
    }

    #[test]
    fn test_merkle_root_different_transactions_different_root() {
        // Different transactions should produce different roots
        let mut private_key = PrivateKey::new_key();

        let tx1 = Transaction::new(vec![], vec![create_test_output(100, &mut private_key)]);
        let tx2 = Transaction::new(vec![], vec![create_test_output(200, &mut private_key)]);

        let transactions1 = vec![tx1.clone()];
        let transactions2 = vec![tx2];

        let root1 = MerkleRoot::calculate(&transactions1);
        let root2 = MerkleRoot::calculate(&transactions2);

        assert_ne!(root1, root2);
    }
}
