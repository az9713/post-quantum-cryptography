//! Blockchain Transaction with WOTS+ Signatures
//!
//! This module implements a simplified transaction structure that uses
//! WOTS+ for quantum-resistant signatures.
//!
//! # Transaction Lifecycle
//! 1. Create unsigned transaction with sender, recipient, amount
//! 2. Compute transaction digest (hash of all fields)
//! 3. Sign the digest with sender's WOTS+ keypair
//! 4. Attach signature and public key to transaction
//! 5. Verify signature before accepting into a block

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use super::{Address, BlockchainError};
use crate::wots::{WotsKeypair, WotsSignature};

/// A blockchain transaction using WOTS+ signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender's address
    pub from: Address,
    /// Recipient's address
    pub to: Address,
    /// Amount to transfer (in smallest units, like satoshis or wei)
    pub amount: u64,
    /// Transaction nonce (prevents replay attacks)
    pub nonce: u64,
    /// Optional transaction data/memo
    pub data: Option<Vec<u8>>,
    /// Signature (None until signed)
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<SerializedSignature>,
    /// Sender's public key (required for verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub_key: Option<Vec<u8>>,
}

/// Serialized signature for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedSignature {
    data: Vec<u8>,
}

impl Transaction {
    /// Create a new unsigned transaction
    pub fn new(from: Address, to: Address, amount: u64, nonce: u64) -> Self {
        Self {
            from,
            to,
            amount,
            nonce,
            data: None,
            signature: None,
            pub_key: None,
        }
    }

    /// Create a transaction with data
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Compute the transaction digest (hash of all unsigned fields)
    ///
    /// Uses SHA3-256 for the digest (not the same as signature hash).
    /// The digest is what gets signed by WOTS+.
    pub fn compute_digest(&self) -> [u8; 32] {
        let mut hasher = Sha3_256::new();

        // Hash all transaction fields in deterministic order
        hasher.update(self.from.as_bytes());
        hasher.update(self.to.as_bytes());
        hasher.update(self.amount.to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());

        if let Some(ref data) = self.data {
            hasher.update((data.len() as u64).to_le_bytes());
            hasher.update(data);
        } else {
            hasher.update(0u64.to_le_bytes());
        }

        let result = hasher.finalize();
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&result);
        digest
    }

    /// Sign the transaction with the sender's keypair
    ///
    /// # Arguments
    /// * `keypair` - The sender's WOTS+ keypair (will be marked as used)
    ///
    /// # Returns
    /// * `Ok(())` if signing succeeded
    /// * `Err(BlockchainError)` if the key was already used
    ///
    /// # Warning
    /// This consumes the one-time use of the keypair. The keypair should
    /// not be used again after this call.
    pub fn sign(&mut self, keypair: &mut WotsKeypair) -> Result<(), BlockchainError> {
        // Verify the keypair matches the from address
        let expected_address = Address::from_keypair(keypair);
        if self.from != expected_address {
            return Err(BlockchainError::InvalidTransaction(
                "Keypair does not match sender address".into(),
            ));
        }

        let digest = self.compute_digest();
        let signature = keypair.sign(&digest)?;

        self.signature = Some(SerializedSignature {
            data: signature.to_bytes(),
        });
        self.pub_key = Some(keypair.public_key_bytes());

        Ok(())
    }

    /// Verify the transaction signature
    ///
    /// # Returns
    /// * `Ok(())` if the signature is valid
    /// * `Err(BlockchainError)` if invalid or missing
    pub fn verify_signature(&self) -> Result<(), BlockchainError> {
        let signature = self
            .signature
            .as_ref()
            .ok_or(BlockchainError::MissingSignature)?;

        let pub_key = self
            .pub_key
            .as_ref()
            .ok_or(BlockchainError::MissingSignature)?;

        // Verify that the public key matches the from address
        let pk_hash = crate::wots::hash(pub_key);
        let expected_from = Address::from_bytes(pk_hash);
        if self.from != expected_from {
            return Err(BlockchainError::InvalidSignature);
        }

        // Deserialize and verify the signature
        let wots_sig = WotsSignature::from_bytes(&signature.data)
            .map_err(|e| BlockchainError::WotsError(e))?;

        let digest = self.compute_digest();
        wots_sig
            .verify(pub_key, &digest)
            .map_err(|_| BlockchainError::InvalidSignature)?;

        Ok(())
    }

    /// Check if the transaction is signed
    pub fn is_signed(&self) -> bool {
        self.signature.is_some() && self.pub_key.is_some()
    }

    /// Get the transaction size in bytes (approximate)
    pub fn size_bytes(&self) -> usize {
        let mut size = 0;

        // Fixed fields
        size += 32; // from address
        size += 32; // to address
        size += 8; // amount
        size += 8; // nonce

        // Optional data
        if let Some(ref data) = self.data {
            size += data.len();
        }

        // Signature and public key
        if let Some(ref sig) = self.signature {
            size += sig.data.len();
        }
        if let Some(ref pk) = self.pub_key {
            size += pk.len();
        }

        size
    }

    /// Get the signature size (if signed)
    pub fn signature_size(&self) -> Option<usize> {
        self.signature.as_ref().map(|s| s.data.len())
    }

    /// Serialize the transaction to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize a transaction from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Transaction size comparison with classical blockchain
pub struct TransactionComparison;

impl TransactionComparison {
    /// Typical Bitcoin transaction size (1 input, 2 outputs)
    pub const BITCOIN_TYPICAL: usize = 225;

    /// Typical Ethereum transaction size
    pub const ETHEREUM_TYPICAL: usize = 110;

    /// Our WOTS+ transaction size (unsigned base + signature + pubkey)
    pub fn wots_transaction_size() -> usize {
        // Base: from(32) + to(32) + amount(8) + nonce(8) = 80 bytes
        // Signature: ~2200 bytes
        // Public key: 2144 bytes
        80 + 2200 + 2144
    }

    /// Explain transaction size implications
    pub fn explain() -> String {
        format!(
            "Transaction Size Comparison:\n\
             \n\
             Bitcoin (1-in, 2-out P2PKH):\n\
             - Total: ~{} bytes\n\
             - Signature: ~72 bytes (DER encoded)\n\
             - Public key: 33 bytes (compressed)\n\
             \n\
             Ethereum:\n\
             - Total: ~{} bytes\n\
             - Signature: 65 bytes (r, s, v)\n\
             - Public key: recovered from signature\n\
             \n\
             WOTS+ (Quantum-Resistant):\n\
             - Total: ~{} bytes (~{}x larger)\n\
             - Signature: ~2144 bytes\n\
             - Public key: ~2144 bytes\n\
             \n\
             This is why STARK aggregation is essential for post-quantum blockchains.\n\
             A single STARK proof (~50KB) can verify thousands of WOTS+ signatures.",
            Self::BITCOIN_TYPICAL,
            Self::ETHEREUM_TYPICAL,
            Self::wots_transaction_size(),
            Self::wots_transaction_size() / Self::BITCOIN_TYPICAL
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let tx = Transaction::new(from.clone(), to.clone(), 1000, 0);

        assert_eq!(tx.from, from);
        assert_eq!(tx.to, to);
        assert_eq!(tx.amount, 1000);
        assert!(!tx.is_signed());
    }

    #[test]
    fn test_transaction_signing() {
        let mut keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let mut tx = Transaction::new(from, to, 1000, 0);

        assert!(tx.sign(&mut keypair1).is_ok());
        assert!(tx.is_signed());
        assert!(keypair1.is_used());
    }

    #[test]
    fn test_transaction_verification() {
        let mut keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let mut tx = Transaction::new(from, to, 1000, 0);
        tx.sign(&mut keypair1).unwrap();

        assert!(tx.verify_signature().is_ok());
    }

    #[test]
    fn test_tampered_transaction_fails() {
        let mut keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let mut tx = Transaction::new(from, to, 1000, 0);
        tx.sign(&mut keypair1).unwrap();

        // Tamper with the amount
        tx.amount = 9999;

        // Verification should fail
        assert!(tx.verify_signature().is_err());
    }

    #[test]
    fn test_wrong_keypair_fails() {
        let keypair1 = WotsKeypair::generate();
        let mut keypair2 = WotsKeypair::generate();
        let keypair3 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair3);

        let mut tx = Transaction::new(from, to, 1000, 0);

        // Try to sign with wrong keypair
        let result = tx.sign(&mut keypair2);
        assert!(result.is_err());
    }

    #[test]
    fn test_digest_deterministic() {
        let keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let tx1 = Transaction::new(from.clone(), to.clone(), 1000, 0);
        let tx2 = Transaction::new(from, to, 1000, 0);

        assert_eq!(tx1.compute_digest(), tx2.compute_digest());
    }

    #[test]
    fn test_json_roundtrip() {
        let mut keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let from = Address::from_keypair(&keypair1);
        let to = Address::from_keypair(&keypair2);

        let mut tx = Transaction::new(from, to, 1000, 0);
        tx.sign(&mut keypair1).unwrap();

        let json = tx.to_json().unwrap();
        let recovered = Transaction::from_json(&json).unwrap();

        assert!(recovered.verify_signature().is_ok());
    }
}
