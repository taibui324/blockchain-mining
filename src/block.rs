use crate::transaction::Transaction;
use crate::error::BlockchainError;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub merkle_root: String,
}

impl Block {
    pub fn new(
        index: u64, 
        transactions: Vec<Transaction>, 
        previous_hash: String,
        difficulty: u32
    ) -> Result<Block, BlockchainError> {
        let mut block = Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            merkle_root: String::new(),
        };
        
        block.merkle_root = block.calculate_merkle_root()?;
        block.mine(difficulty)?;
        Ok(block)
    }

    pub fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.merkle_root,
            self.previous_hash,
            self.nonce,
            self.transactions.iter()
                .map(|tx| tx.id.clone())
                .collect::<Vec<String>>()
                .join("")
        );
        
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn calculate_merkle_root(&self) -> Result<String, BlockchainError> {
        if self.transactions.is_empty() {
            return Ok(String::from("0"));
        }

        let mut hashes: Vec<String> = self.transactions
            .iter()
            .map(|tx| tx.calculate_hash())
            .collect();

        while hashes.len() > 1 {
            let mut temp_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let mut hasher = Sha256::new();
                if chunk.len() == 2 {
                    hasher.update(format!("{}{}", chunk[0], chunk[1]).as_bytes());
                } else {
                    hasher.update(format!("{}{}", chunk[0], chunk[0]).as_bytes());
                }
                temp_hashes.push(format!("{:x}", hasher.finalize()));
            }
            hashes = temp_hashes;
        }

        Ok(hashes[0].clone())
    }

    pub fn mine(&mut self, difficulty: u32) -> Result<(), BlockchainError> {
        let target = "0".repeat(difficulty as usize);
        
        while !self.calculate_hash().starts_with(&target) {
            self.nonce += 1;
            if self.nonce == u64::MAX {
                return Err(BlockchainError::MiningError("Nonce overflow".to_string()));
            }
        }
        
        self.hash = self.calculate_hash();
        Ok(())
    }
} 