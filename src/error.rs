#[derive(Debug)]
pub enum BlockchainError {
    MiningError(String),
    ValidationError(String),
    SignatureError(String),
    WalletError(String),
    TransactionError(String),
}

impl std::fmt::Display for BlockchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockchainError::MiningError(msg) => write!(f, "Mining error: {}", msg),
            BlockchainError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            BlockchainError::SignatureError(msg) => write!(f, "Signature error: {}", msg),
            BlockchainError::WalletError(msg) => write!(f, "Wallet error: {}", msg),
            BlockchainError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
} 