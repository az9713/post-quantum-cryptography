//! WOTS+ Signature Verification
//!
//! # Verification Algorithm
//! Given a signature (partial chain values) and public key (chain endpoints):
//! 1. Encode the message as base-w digits
//! 2. Compute the checksum and encode it
//! 3. For each signature chain value, hash it (w-1-index) times
//! 4. The result should equal the corresponding public key component
//!
//! If the signature was created by someone with the private key, hashing
//! the signature values forward will reconstruct the public key exactly.

use serde::{Deserialize, Serialize};

use super::{chain_hash, hash, WotsError, WotsParams};

/// A WOTS+ signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WotsSignature {
    /// Signature chains (partial hash chain values)
    chains: Vec<[u8; 32]>,
    /// Winternitz parameter used
    w: usize,
    /// The digest that was signed (for verification)
    signed_digest: Vec<u8>,
}

impl WotsSignature {
    /// Create a new signature (called internally by WotsKeypair::sign)
    pub(crate) fn new(chains: Vec<[u8; 32]>, w: usize, signed_digest: Vec<u8>) -> Self {
        Self {
            chains,
            w,
            signed_digest,
        }
    }

    /// Verify this signature against a public key and message digest
    ///
    /// # Arguments
    /// * `public_key` - The WOTS+ public key (chain endpoints)
    /// * `digest` - The message digest that was supposedly signed
    ///
    /// # Returns
    /// * `Ok(())` if the signature is valid
    /// * `Err(WotsError::VerificationFailed)` if invalid
    pub fn verify(&self, public_key: &[u8], digest: &[u8; 32]) -> Result<(), WotsError> {
        // Check that the digest matches what was signed
        if self.signed_digest != digest {
            return Err(WotsError::VerificationFailed);
        }

        let params = self.derive_params();

        // Convert public key bytes to chain endpoints
        let pk_chains: Vec<[u8; 32]> = public_key
            .chunks_exact(32)
            .map(|chunk| {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(chunk);
                arr
            })
            .collect();

        if pk_chains.len() != params.total_chains() {
            return Err(WotsError::InvalidSignature(format!(
                "Public key has {} chains, expected {}",
                pk_chains.len(),
                params.total_chains()
            )));
        }

        if self.chains.len() != params.total_chains() {
            return Err(WotsError::InvalidSignature(format!(
                "Signature has {} chains, expected {}",
                self.chains.len(),
                params.total_chains()
            )));
        }

        // Encode message and checksum
        let msg_indices = self.base_w_encode(digest, &params);
        let checksum = self.compute_checksum(&msg_indices, &params);
        let checksum_indices = self.encode_checksum(checksum, &params);

        let mut all_indices = msg_indices;
        all_indices.extend(checksum_indices);

        // Verify each chain: hash signature value (w-1-index) times, should equal PK
        for (i, (sig_chain, pk_chain)) in self.chains.iter().zip(pk_chains.iter()).enumerate() {
            let index = all_indices[i];
            let remaining_hashes = params.w - 1 - index;
            let computed = chain_hash(sig_chain, remaining_hashes);

            if computed != *pk_chain {
                return Err(WotsError::VerificationFailed);
            }
        }

        Ok(())
    }

    /// Get the signature size in bytes
    pub fn size_bytes(&self) -> usize {
        self.chains.len() * 32
    }

    /// Serialize the signature to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.size_bytes() + 1 + self.signed_digest.len());

        // Winternitz parameter (1 byte)
        result.push(self.w as u8);

        // Signature chains
        for chain in &self.chains {
            result.extend_from_slice(chain);
        }

        // Signed digest
        result.extend_from_slice(&self.signed_digest);

        result
    }

    /// Deserialize a signature from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, WotsError> {
        if data.len() < 33 {
            return Err(WotsError::InvalidSignature("Data too short".into()));
        }

        let w = data[0] as usize;
        let params = WotsParams {
            w,
            n: 32,
            l1: if w == 16 { 64 } else { 32 },
            l2: 3,
        };

        let expected_chains = params.total_chains();
        let chains_size = expected_chains * 32;

        if data.len() < 1 + chains_size + 32 {
            return Err(WotsError::InvalidSignature("Data too short for chains".into()));
        }

        let chains: Vec<[u8; 32]> = data[1..1 + chains_size]
            .chunks_exact(32)
            .map(|chunk| {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(chunk);
                arr
            })
            .collect();

        let signed_digest = data[1 + chains_size..].to_vec();

        Ok(Self {
            chains,
            w,
            signed_digest,
        })
    }

    /// Derive WOTS parameters from the signature
    fn derive_params(&self) -> WotsParams {
        WotsParams {
            w: self.w,
            n: 32,
            l1: if self.w == 16 { 64 } else { 32 },
            l2: 3,
        }
    }

    /// Convert digest to base-w representation (same as in keypair)
    fn base_w_encode(&self, digest: &[u8; 32], params: &WotsParams) -> Vec<usize> {
        let mut result = Vec::with_capacity(params.l1);

        match params.w {
            16 => {
                for byte in digest {
                    result.push((byte >> 4) as usize);
                    result.push((byte & 0x0F) as usize);
                }
            }
            256 => {
                for byte in digest {
                    result.push(*byte as usize);
                }
            }
            _ => {
                for byte in digest {
                    result.push((byte >> 4) as usize);
                    result.push((byte & 0x0F) as usize);
                }
            }
        }

        result.truncate(params.l1);
        result
    }

    /// Compute checksum (same as in keypair)
    fn compute_checksum(&self, msg_indices: &[usize], params: &WotsParams) -> usize {
        let max_digit = params.w - 1;
        msg_indices.iter().map(|&d| max_digit - d).sum()
    }

    /// Encode checksum as base-w digits (same as in keypair)
    fn encode_checksum(&self, checksum: usize, params: &WotsParams) -> Vec<usize> {
        let mut result = Vec::with_capacity(params.l2);
        let mut remaining = checksum;

        for _ in 0..params.l2 {
            result.push(remaining % params.w);
            remaining /= params.w;
        }

        result.reverse();
        result
    }
}

/// Comparison to ECDSA signature sizes
pub struct SignatureComparison;

impl SignatureComparison {
    /// ECDSA signature size (r, s components)
    pub const ECDSA_SIZE: usize = 64;

    /// Typical WOTS+ signature size with w=16
    pub const WOTS_SIZE: usize = 2144;

    /// Compression ratio (how much larger WOTS+ is)
    pub fn compression_ratio() -> f64 {
        Self::WOTS_SIZE as f64 / Self::ECDSA_SIZE as f64
    }

    /// Explain the size difference
    pub fn explain() -> String {
        format!(
            "WOTS+ signatures are ~{}x larger than ECDSA:\n\
             - ECDSA: {} bytes (r,s components)\n\
             - WOTS+ (w=16): {} bytes ({} chains x 32 bytes)\n\n\
             This is the trade-off for quantum resistance.\n\
             STARKs can aggregate many WOTS+ signatures into a single ~50KB proof.",
            Self::compression_ratio() as usize,
            Self::ECDSA_SIZE,
            Self::WOTS_SIZE,
            67
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wots::WotsKeypair;

    #[test]
    fn test_sign_and_verify() {
        let mut keypair = WotsKeypair::generate();
        let message = b"Hello, post-quantum world!";
        let digest = hash(message);

        let signature = keypair.sign(&digest).unwrap();
        let public_key = keypair.public_key_bytes();

        assert!(signature.verify(&public_key, &digest).is_ok());
    }

    #[test]
    fn test_verification_fails_with_wrong_digest() {
        let mut keypair = WotsKeypair::generate();
        let message = b"Original message";
        let digest = hash(message);

        let signature = keypair.sign(&digest).unwrap();
        let public_key = keypair.public_key_bytes();

        // Try to verify with different message
        let wrong_digest = hash(b"Different message");
        assert!(signature.verify(&public_key, &wrong_digest).is_err());
    }

    #[test]
    fn test_verification_fails_with_wrong_key() {
        let mut keypair1 = WotsKeypair::generate();
        let keypair2 = WotsKeypair::generate();

        let digest = hash(b"test");
        let signature = keypair1.sign(&digest).unwrap();

        // Try to verify with different public key
        let wrong_pk = keypair2.public_key_bytes();
        assert!(signature.verify(&wrong_pk, &digest).is_err());
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut keypair = WotsKeypair::generate();
        let digest = hash(b"serialize me");

        let signature = keypair.sign(&digest).unwrap();
        let bytes = signature.to_bytes();
        let recovered = WotsSignature::from_bytes(&bytes).unwrap();

        // Should verify the same way
        let public_key = keypair.public_key_bytes();
        assert!(recovered.verify(&public_key, &digest).is_ok());
    }

    #[test]
    fn test_signature_size() {
        let mut keypair = WotsKeypair::generate();
        let digest = hash(b"test");
        let signature = keypair.sign(&digest).unwrap();

        assert_eq!(signature.size_bytes(), 2144);
    }
}
