use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid transaction: {reason}")]
    InvalidTransaction { reason: String },
    #[error("Invalid block: {reason}")]
    InvalidBlock { reason: String },
    #[error("Invalid block header: {reason}")]
    InvalidBlockHeader { reason: String },
    #[error("Invalid transaction input: {reason}")]
    InvalidTransactionInput { reason: String },
    #[error("Invalid transaction output: {reason}")]
    InvalidTransactionOutput { reason: String },
    #[error("Invalid Merkle root: calculated root does not match block header")]
    InvalidMerkleRoot,
    #[error("Invalid hash: {reason}")]
    InvalidHash { reason: String },
    #[error("Invalid signature: signature verification failed")]
    InvalidSignature,
    #[error("Invalid public key: {reason}")]
    InvalidPublicKey { reason: String },
    #[error("Invalid private key: {reason}")]
    InvalidPrivateKey { reason: String },
}

// Convenience methods for creating errors
impl BtcError {
    pub fn invalid_transaction<S: Into<String>>(reason: S) -> Self {
        BtcError::InvalidTransaction {
            reason: reason.into(),
        }
    }

    pub fn invalid_block<S: Into<String>>(reason: S) -> Self {
        BtcError::InvalidBlock {
            reason: reason.into(),
        }
    }

    pub fn invalid_hash<S: Into<String>>(reason: S) -> Self {
        BtcError::InvalidHash {
            reason: reason.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, BtcError>;
