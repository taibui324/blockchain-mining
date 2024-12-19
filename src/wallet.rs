use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use crate::error::BlockchainError;
use base58::{ToBase58, FromBase58};

#[derive(Debug)]
pub struct Wallet {
    keypair: Keypair,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Result<Self, BlockchainError> {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public.to_bytes().to_base58();

        Ok(Wallet {
            keypair,
            public_key,
        })
    }

    pub fn sign(&self, message: &str) -> Result<String, BlockchainError> {
        let signature = self.keypair.sign(message.as_bytes());
        Ok(signature.to_bytes().to_base58())
    }

    pub fn verify(public_key: &str, message: &str, signature: &str) -> Result<bool, BlockchainError> {
        let public_key_bytes = public_key.from_base58()
            .map_err(|e| BlockchainError::WalletError(format!("Invalid public key: {:?}", e)))?;
        let signature_bytes = signature.from_base58()
            .map_err(|e| BlockchainError::WalletError(format!("Invalid signature: {:?}", e)))?;

        let public_key = PublicKey::from_bytes(&public_key_bytes)
            .map_err(|e| BlockchainError::WalletError(format!("Invalid public key format: {}", e)))?;
        let signature = Signature::from_bytes(&signature_bytes)
            .map_err(|e| BlockchainError::WalletError(format!("Invalid signature format: {}", e)))?;

        Ok(public_key.verify(message.as_bytes(), &signature).is_ok())
    }
} 