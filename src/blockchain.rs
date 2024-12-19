use crate::{
    block::Block,
    transaction::Transaction,
    error::BlockchainError,
    wallet::Wallet,
};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Result<Self, BlockchainError> {
        let mut chain = Vec::new();
        
        // Create genesis block
        let genesis_block = Block::new(
            0,
            Vec::new(),
            String::from("0"),
            difficulty,
        )?;
        
        chain.push(genesis_block);

        Ok(Blockchain {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
            mining_reward: 50.0,
        })
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), BlockchainError> {
        // Validate transaction
        if !transaction.is_valid()? {
            return Err(BlockchainError::ValidationError(
                "Invalid transaction".to_string()
            ));
        }

        // Add to pending transactions
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn mine_pending_transactions(&mut self, miner_reward_address: String) -> Result<(), BlockchainError> {
        // Create mining reward transaction
        let reward_tx = Transaction::new(
            &Wallet::new()?, // System wallet
            miner_reward_address,
            self.mining_reward,
        )?;

        // Add reward transaction to pending transactions
        self.pending_transactions.push(reward_tx);

        // Create new block with pending transactions
        let block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.get_latest_block()?.hash.clone(),
            self.difficulty,
        )?;

        // Add block to chain
        self.chain.push(block);
        
        // Clear pending transactions
        self.pending_transactions = Vec::new();
        
        Ok(())
    }

    pub fn get_latest_block(&self) -> Result<&Block, BlockchainError> {
        self.chain.last().ok_or(BlockchainError::ValidationError(
            "Empty blockchain".to_string()
        ))
    }

    pub fn is_chain_valid(&self) -> Result<bool, BlockchainError> {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Verify all transactions in block
            for transaction in &current_block.transactions {
                if !transaction.is_valid()? {
                    return Ok(false);
                }
            }

            // Verify block hash
            if current_block.hash != current_block.calculate_hash() {
                return Ok(false);
            }

            // Verify chain continuity
            if current_block.previous_hash != previous_block.hash {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn get_balance(&self, address: &str) -> Result<f64, BlockchainError> {
        let mut balance = 0.0;

        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.from == address {
                    balance -= transaction.amount;
                }
                if transaction.to == address {
                    balance += transaction.amount;
                }
            }
        }

        Ok(balance)
    }
} 