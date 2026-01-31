//! WOTS+ (Winternitz One-Time Signatures Plus) Implementation
//!
//! This module provides a quantum-resistant digital signature scheme based on hash chains.
//!
//! # Security Model
//! WOTS+ security relies solely on the pre-image and second pre-image resistance of the
//! underlying hash function (Blake3). Unlike ECDSA which is broken by Shor's algorithm,
//! hash-based signatures remain secure against quantum computers (only √N speedup from
//! Grover's algorithm vs exponential speedup from Shor).
//!
//! # One-Time Property
//! **CRITICAL**: Each WOTS+ keypair can only sign ONE message safely. Signing multiple
//! messages reveals additional hash chain preimages, potentially enabling signature forgery.

mod keypair;
mod signature;

pub use keypair::{WotsKeypair, WotsPublicKey};
pub use signature::WotsSignature;

use thiserror::Error;

/// WOTS+ configuration parameters
#[derive(Debug, Clone)]
pub struct WotsParams {
    /// Winternitz parameter (base for message encoding)
    /// Higher w = smaller signatures but slower operations
    /// Common values: 4, 16, 256
    pub w: usize,
    /// Security parameter in bits (determines hash output size)
    pub n: usize,
    /// Number of message chains (derived from n and w)
    pub l1: usize,
    /// Number of checksum chains (derived from l1 and w)
    pub l2: usize,
}

impl Default for WotsParams {
    fn default() -> Self {
        // w=16 provides good balance between signature size and speed
        // n=32 bytes = 256 bits security
        let w = 16;
        let n = 32;
        // l1 = ceil(8n / log2(w)) = ceil(256 / 4) = 64
        let l1 = 64;
        // l2 = floor(log2(l1 * (w-1)) / log2(w)) + 1 = floor(log2(64*15)/4) + 1 = 3
        let l2 = 3;

        Self { w, n, l1, l2 }
    }
}

impl WotsParams {
    /// Total number of hash chains (message + checksum)
    pub fn total_chains(&self) -> usize {
        self.l1 + self.l2
    }

    /// Size of public key in bytes
    pub fn public_key_size(&self) -> usize {
        self.total_chains() * self.n
    }

    /// Size of signature in bytes
    pub fn signature_size(&self) -> usize {
        self.total_chains() * self.n
    }
}

/// Errors that can occur during WOTS+ operations
#[derive(Error, Debug)]
pub enum WotsError {
    #[error("Key has already been used for signing - WOTS+ keys are ONE-TIME only")]
    KeyAlreadyUsed,

    #[error("Signature verification failed - signature does not match public key")]
    VerificationFailed,

    #[error("Invalid signature format: {0}")]
    InvalidSignature(String),

    #[error("Invalid message digest length: expected {expected}, got {got}")]
    InvalidDigestLength { expected: usize, got: usize },
}

/// Hash a value using Blake3 (quantum-resistant hash function)
pub fn hash(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

/// Chain hash: apply hash function `iterations` times
pub(crate) fn chain_hash(input: &[u8; 32], iterations: usize) -> [u8; 32] {
    let mut result = *input;
    for _ in 0..iterations {
        result = hash(&result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = WotsParams::default();
        assert_eq!(params.w, 16);
        assert_eq!(params.n, 32);
        assert_eq!(params.l1, 64);
        assert_eq!(params.l2, 3);
        assert_eq!(params.total_chains(), 67);
        // 67 chains * 32 bytes = 2144 bytes
        assert_eq!(params.signature_size(), 2144);
    }

    #[test]
    fn test_chain_hash() {
        let input = hash(b"test");
        let chained = chain_hash(&input, 5);

        // Verify manual chaining produces same result
        let mut manual = input;
        for _ in 0..5 {
            manual = hash(&manual);
        }
        assert_eq!(chained, manual);
    }
}
