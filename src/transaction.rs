use sha2::{Sha256, Digest};
use uuid::Uuid;
use crate::wallet::Wallet;
use crate::error::BlockchainError;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub signature: Option<String>,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(
        from_wallet: &Wallet,
        to_address: String,
        amount: f64
    ) -> Result<Transaction, BlockchainError> {
        let mut transaction = Transaction {
            id: Uuid::new_v4().to_string(),
            from: from_wallet.public_key.clone(),
            to: to_address,
            amount,
            signature: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        transaction.sign(from_wallet)?;
        Ok(transaction)
    }

    pub fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}",
            self.id,
            self.from,
            self.to,
            self.amount,
            self.timestamp
        );
        
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn sign(&mut self, wallet: &Wallet) -> Result<(), BlockchainError> {
        if wallet.public_key != self.from {
            return Err(BlockchainError::SignatureError(
                "Cannot sign transaction for other wallets".to_string()
            ));
        }

        let hash = self.calculate_hash();
        self.signature = Some(wallet.sign(&hash)?);
        Ok(())
    }

    pub fn is_valid(&self) -> Result<bool, BlockchainError> {
        if self.from == "0" {
            return Ok(true); // Mining reward transaction
        }

        match &self.signature {
            None => Err(BlockchainError::ValidationError("No signature".to_string())),
            Some(signature) => {
                let hash = self.calculate_hash();
                Wallet::verify(&self.from, &hash, signature)
            }
        }
    }
} 