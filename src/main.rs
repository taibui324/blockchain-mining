use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

// Block structure representing a single block in the blockchain
#[derive(Debug, Clone)]
struct Block {
    // Index of the block in the chain
    index: u64,
    // Timestamp of when the block was created
    timestamp: u64,
    // Data stored in the block (in a real blockchain, this could be transactions)
    data: String,
    // Hash of the previous block (creates the chain)
    previous_hash: String,
    // Hash of the current block
    hash: String,
    // Nonce used for mining (proof of work)
    nonce: u64,
}

impl Block {
    // Create a new block
    fn new(index: u64, data: String, previous_hash: String, difficulty: u32) -> Block {
        let mut block = Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        
        // Mine the block with specified difficulty
        block.mine(difficulty);
        block
    }

    // Calculate hash of the block using SHA256
    fn calculate_hash(&self) -> String {
        let content = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    // Mine the block (proof of work)
    fn mine(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);

        // Keep incrementing nonce until we find a hash with required difficulty
        while !self.calculate_hash().starts_with(&target) {
            self.nonce += 1;
        }
        
        self.hash = self.calculate_hash();
        println!("Block mined! Nonce: {}", self.nonce); // Added mining feedback
    }
}

// Blockchain structure to manage the chain of blocks
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
}

impl Blockchain {
    // Create a new blockchain with genesis block
    fn new() -> Blockchain {
        let mut chain = Vec::new();
        let difficulty = 4; // Default difficulty

        // Create genesis block with difficulty
        chain.push(Block::new(
            0,
            String::from("Genesis Block"),
            String::from("0"),
            difficulty,
        ));

        Blockchain {
            chain,
            difficulty,
        }
    }

    // Add a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    // Verify the integrity of the blockchain
    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Verify current block's hash
            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash for block {}", current_block.index);
                return false;
            }

            // Verify chain continuity
            if current_block.previous_hash != previous_block.hash {
                println!("Chain broken at block {}", current_block.index);
                return false;
            }
        }
        true
    }

    // Add method to adjust difficulty
    fn adjust_difficulty(&mut self, new_difficulty: u32) {
        self.difficulty = new_difficulty;
        println!("Difficulty adjusted to: {}", self.difficulty);
    }
}

fn main() {
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    println!("Created blockchain with genesis block");

    // Add some blocks with default difficulty
    println!("Mining block 1...");
    blockchain.add_block(String::from("First Block Data"));
    
    // Adjust difficulty and mine another block
    blockchain.adjust_difficulty(5); // Increase difficulty
    println!("Mining block 2 with increased difficulty...");
    blockchain.add_block(String::from("Second Block Data"));

    // Verify the blockchain
    println!("Is blockchain valid? {}", blockchain.is_valid());

    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("Block #{}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Data: {}", block.data);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("------------------------");
    }
}
