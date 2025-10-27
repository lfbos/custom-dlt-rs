#[cfg(test)]
mod tests {
    use crate::crypto::{PrivateKey, Signature};
    use crate::sha256::Hash;

    #[test]
    fn test_key_generation() {
        let private_key = PrivateKey::new_key();
        let public_key = private_key.public_key();

        // Keys should be generated successfully
        assert_ne!(private_key.0.to_bytes().len(), 0);

        // Public key should be derivable from private key
        let public_key2 = private_key.public_key();
        assert_eq!(public_key, public_key2);
    }

    #[test]
    fn test_public_key_derivation() {
        let private_key = PrivateKey::new_key();
        let public_key1 = private_key.public_key();
        let public_key2 = private_key.public_key();

        // Same private key should produce same public key
        assert_eq!(public_key1, public_key2);
    }

    #[test]
    fn test_signature_creation_and_verification() {
        let mut private_key = PrivateKey::new_key();
        let public_key = private_key.public_key();

        let message_hash = Hash::hash(&"test message");

        let signature = Signature::sign_output(&message_hash, &mut private_key);

        // Signature should verify correctly
        let is_valid = signature.verify(&message_hash, &public_key);
        assert!(is_valid);
    }

    #[test]
    fn test_signature_verification_fails_wrong_message() {
        let mut private_key = PrivateKey::new_key();
        let public_key = private_key.public_key();

        let message1 = Hash::hash(&"message 1");
        let message2 = Hash::hash(&"message 2");

        let signature = Signature::sign_output(&message1, &mut private_key);

        // Signature should NOT verify for different message
        let is_valid = signature.verify(&message2, &public_key);
        assert!(!is_valid);
    }

    #[test]
    fn test_signature_verification_fails_wrong_key() {
        let mut private_key1 = PrivateKey::new_key();
        let private_key2 = PrivateKey::new_key();
        let public_key2 = private_key2.public_key();

        let message = Hash::hash(&"test message");

        let signature = Signature::sign_output(&message, &mut private_key1);

        // Signature should NOT verify with wrong public key
        let is_valid = signature.verify(&message, &public_key2);
        assert!(!is_valid);
    }
}
