//! Address Derivation from WOTS+ Public Keys
//!
//! Addresses are derived by hashing the public key, similar to how
//! Bitcoin and Ethereum derive addresses from ECDSA public keys.
//!
//! # Address Format
//! - Full public key: 2144 bytes (too large to use directly)
//! - Address: 32 bytes (Blake3 hash of public key)
//! - Display format: "qw1" prefix + hex encoding (like "bc1" for Bitcoin SegWit)

use serde::{Deserialize, Serialize};

use crate::wots::{WotsKeypair, WotsPublicKey};

/// A quantum-resistant blockchain address
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Address {
    /// The 32-byte address hash
    bytes: [u8; 32],
}

impl Address {
    /// Derive an address from a WOTS+ keypair
    pub fn from_keypair(keypair: &WotsKeypair) -> Self {
        Self::from_public_key(&keypair.public_key())
    }

    /// Derive an address from a WOTS+ public key
    pub fn from_public_key(public_key: &WotsPublicKey) -> Self {
        Self {
            bytes: public_key.hash(),
        }
    }

    /// Create an address from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    /// Parse an address from its string representation
    pub fn from_string(s: &str) -> Result<Self, &'static str> {
        if !s.starts_with("qw1") {
            return Err("Address must start with 'qw1' prefix");
        }

        let hex_part = &s[3..];
        let bytes = hex::decode(hex_part).map_err(|_| "Invalid hex encoding")?;

        if bytes.len() != 32 {
            return Err("Address must be 32 bytes");
        }

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Self { bytes: arr })
    }

    /// Get the raw bytes of the address
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// Convert to string representation with "qw1" prefix
    /// (quantum-winternitz-1, similar to Bitcoin's "bc1" for bech32)
    pub fn to_string(&self) -> String {
        format!("qw1{}", hex::encode(self.bytes))
    }

    /// Get a shortened display version (first and last 8 chars)
    pub fn short_display(&self) -> String {
        let full = self.to_string();
        if full.len() <= 20 {
            full
        } else {
            format!("{}...{}", &full[..11], &full[full.len() - 8..])
        }
    }

    /// Create a zero address (often used for coinbase/mint transactions)
    pub fn zero() -> Self {
        Self { bytes: [0u8; 32] }
    }

    /// Check if this is the zero address
    pub fn is_zero(&self) -> bool {
        self.bytes == [0u8; 32]
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Comparison: Address sizes across different schemes
pub struct AddressComparison;

impl AddressComparison {
    /// Bitcoin P2PKH address (20 bytes, displayed as base58)
    pub const BITCOIN_P2PKH: usize = 20;

    /// Ethereum address (20 bytes, displayed as hex with 0x prefix)
    pub const ETHEREUM: usize = 20;

    /// Our WOTS+ address (32 bytes for full security)
    pub const WOTS_ADDRESS: usize = 32;

    /// Explain address derivation
    pub fn explain() -> String {
        format!(
            "Address Derivation Comparison:\n\
             \n\
             Bitcoin (P2PKH):\n\
             - Public Key: 33 bytes (compressed secp256k1)\n\
             - Address: {} bytes = RIPEMD160(SHA256(pubkey))\n\
             \n\
             Ethereum:\n\
             - Public Key: 64 bytes (uncompressed secp256k1)\n\
             - Address: {} bytes = last 20 bytes of Keccak256(pubkey)\n\
             \n\
             WOTS+ (Quantum-Resistant):\n\
             - Public Key: 2144 bytes (67 x 32-byte chain endpoints)\n\
             - Address: {} bytes = Blake3(pubkey)\n\
             \n\
             The larger public key is hidden in the address hash, but must be\n\
             revealed when spending. This makes the first transaction from an\n\
             address the most critical for quantum security.",
            Self::BITCOIN_P2PKH,
            Self::ETHEREUM,
            Self::WOTS_ADDRESS
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_from_keypair() {
        let keypair = WotsKeypair::generate();
        let address = Address::from_keypair(&keypair);

        // Address should be deterministic from keypair
        let address2 = Address::from_keypair(&keypair);
        assert_eq!(address, address2);
    }

    #[test]
    fn test_address_string_roundtrip() {
        let keypair = WotsKeypair::generate();
        let address = Address::from_keypair(&keypair);

        let string = address.to_string();
        assert!(string.starts_with("qw1"));

        let parsed = Address::from_string(&string).unwrap();
        assert_eq!(address, parsed);
    }

    #[test]
    fn test_different_keypairs_different_addresses() {
        let keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let addr1 = Address::from_keypair(&keypair1);
        let addr2 = Address::from_keypair(&keypair2);

        assert_ne!(addr1, addr2);
    }

    #[test]
    fn test_short_display() {
        let keypair = WotsKeypair::generate();
        let address = Address::from_keypair(&keypair);

        let short = address.short_display();
        // Should be truncated with "..."
        assert!(short.contains("..."));
        assert!(short.len() < address.to_string().len());
    }

    #[test]
    fn test_zero_address() {
        let zero = Address::zero();
        assert!(zero.is_zero());

        let keypair = WotsKeypair::generate();
        let regular = Address::from_keypair(&keypair);
        assert!(!regular.is_zero());
    }
}
