use blockchain_rust::{
    wallet::Wallet,
    transaction::Transaction,
    blockchain::Blockchain,
    error::BlockchainError,
};

fn main() -> Result<(), BlockchainError> {
    println!("Creating new blockchain...");
    let mut blockchain = Blockchain::new(4)?; // Initialize with difficulty 4
    
    // Create wallets for testing
    println!("\nCreating wallets...");
    let miner_wallet = Wallet::new()?;
    let alice_wallet = Wallet::new()?;
    let bob_wallet = Wallet::new()?;
    
    println!("Tai's address: {}", miner_wallet.public_key);
    println!("Dory's address: {}", alice_wallet.public_key);
    println!("Mochi's address: {}", bob_wallet.public_key);

    // Create and add mining reward transaction
    println!("\nCreating mining reward transaction...");
    let mining_reward = Transaction::new(
        &Wallet::new()?, // System wallet for rewards
        miner_wallet.public_key.clone(),
        50.0, // Mining reward amount
    )?;

    // Create a transaction from Alice to Bob
    println!("\nCreating transaction: Alice -> Bob");
    let transaction1 = Transaction::new(
        &alice_wallet,
        bob_wallet.public_key.clone(),
        10.0,
    )?;

    // Add transactions to pending list and mine a new block
    println!("\nMining new block with transactions...");
    blockchain.add_transaction(mining_reward)?;
    blockchain.add_transaction(transaction1)?;
    blockchain.mine_pending_transactions(miner_wallet.public_key.clone())?;

    // Create another transaction from Bob to Alice
    println!("\nCreating transaction: Bob -> Alice");
    let transaction2 = Transaction::new(
        &bob_wallet,
        alice_wallet.public_key.clone(),
        5.0,
    )?;

    // Mine another block
    println!("\nMining another block...");
    blockchain.add_transaction(transaction2)?;
    blockchain.mine_pending_transactions(miner_wallet.public_key.clone())?;

    // Verify the blockchain
    println!("\nVerifying blockchain...");
    match blockchain.is_chain_valid() {
        Ok(true) => println!("Blockchain is valid!"),
        Ok(false) => println!("Blockchain is invalid!"),
        Err(e) => println!("Error verifying blockchain: {}", e),
    }

    // Print blockchain state
    println!("\nFinal Blockchain State:");
    println!("Number of blocks: {}", blockchain.chain.len());
    
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("\nBlock #{}", i);
        println!("Timestamp: {}", block.timestamp);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("Merkle Root: {}", block.merkle_root);
        
        println!("Transactions:");
        for tx in &block.transactions {
            println!("  From: {}...", &tx.from[..10]);
            println!("  To: {}...", &tx.to[..10]);
            println!("  Amount: {}", tx.amount);
            println!("  ID: {}", tx.id);
            println!("  Signature: {}", tx.signature.as_ref().unwrap_or(&"None".to_string()));
            println!();
        }
    }

    // Get account balances
    println!("\nFinal Balances:");
    println!("Tai's balance: {}", blockchain.get_balance(&miner_wallet.public_key)?);
    println!("Dory's balance: {}", blockchain.get_balance(&alice_wallet.public_key)?);
    println!("Mochi's balance: {}", blockchain.get_balance(&bob_wallet.public_key)?);

    Ok(())
}
