//! WOTS+ Keypair Generation and Signing
//!
//! The keypair consists of:
//! - Private key: Random seeds for each hash chain
//! - Public key: Final values of each hash chain (after w-1 hashes)
//!
//! # Key Generation Algorithm
//! 1. Generate l random 32-byte seeds (private key chains)
//! 2. For each seed, apply H^(w-1) to get public key component
//! 3. Public key = concatenation of all chain endpoints

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{chain_hash, hash, WotsError, WotsParams, WotsSignature};

/// A WOTS+ keypair for signing messages
///
/// **WARNING**: This keypair can only sign ONE message. After signing,
/// the `used` flag is set and subsequent signing attempts will fail.
#[derive(Debug, Clone)]
pub struct WotsKeypair {
    /// Private key: seeds for each hash chain
    private_key: Vec<[u8; 32]>,
    /// Public key: endpoints of each hash chain
    public_key: Vec<[u8; 32]>,
    /// Whether this key has been used for signing
    used: bool,
    /// WOTS+ parameters
    params: WotsParams,
}

/// Serializable public key for storage/transmission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WotsPublicKey {
    /// Concatenated chain endpoints
    pub data: Vec<u8>,
    /// Winternitz parameter used
    pub w: usize,
}

impl WotsKeypair {
    /// Generate a new WOTS+ keypair with default parameters
    ///
    /// Uses cryptographically secure random number generation for private key seeds.
    pub fn generate() -> Self {
        Self::generate_with_params(WotsParams::default())
    }

    /// Generate a new WOTS+ keypair with custom parameters
    pub fn generate_with_params(params: WotsParams) -> Self {
        let mut rng = rand::thread_rng();
        let total_chains = params.total_chains();

        // Generate random seeds for private key
        let mut private_key = Vec::with_capacity(total_chains);
        for _ in 0..total_chains {
            let mut seed = [0u8; 32];
            rng.fill(&mut seed);
            private_key.push(seed);
        }

        // Derive public key by hashing each seed w-1 times
        let public_key: Vec<[u8; 32]> = private_key
            .iter()
            .map(|seed| chain_hash(seed, params.w - 1))
            .collect();

        Self {
            private_key,
            public_key,
            used: false,
            params,
        }
    }

    /// Sign a message digest
    ///
    /// # Arguments
    /// * `digest` - A 32-byte hash of the message to sign
    ///
    /// # Returns
    /// * `Ok(WotsSignature)` - The signature
    /// * `Err(WotsError::KeyAlreadyUsed)` - If this key was already used
    ///
    /// # Security Warning
    /// After calling this method successfully, the keypair is marked as used
    /// and cannot sign again. This is a fundamental security property of WOTS+.
    pub fn sign(&mut self, digest: &[u8; 32]) -> Result<WotsSignature, WotsError> {
        if self.used {
            return Err(WotsError::KeyAlreadyUsed);
        }

        // Convert digest to base-w representation
        let msg_indices = self.base_w_encode(digest);

        // Calculate checksum
        let checksum = self.compute_checksum(&msg_indices);
        let checksum_indices = self.encode_checksum(checksum);

        // Combine message and checksum indices
        let mut all_indices = msg_indices;
        all_indices.extend(checksum_indices);

        // Generate signature: for each chain, hash the seed `index` times
        let signature_chains: Vec<[u8; 32]> = self
            .private_key
            .iter()
            .zip(all_indices.iter())
            .map(|(seed, &index)| chain_hash(seed, index))
            .collect();

        // Mark key as used
        self.used = true;

        Ok(WotsSignature::new(
            signature_chains,
            self.params.w,
            digest.to_vec(),
        ))
    }

    /// Check if this keypair has been used for signing
    pub fn is_used(&self) -> bool {
        self.used
    }

    /// Get the public key
    pub fn public_key(&self) -> WotsPublicKey {
        WotsPublicKey {
            data: self.public_key.iter().flatten().copied().collect(),
            w: self.params.w,
        }
    }

    /// Get the public key as raw bytes
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.iter().flatten().copied().collect()
    }

    /// Get the size of the public key in bytes
    pub fn public_key_size(&self) -> usize {
        self.params.public_key_size()
    }

    /// Get the WOTS+ parameters
    pub fn params(&self) -> &WotsParams {
        &self.params
    }

    /// Convert a digest to base-w representation
    ///
    /// For w=16, each nibble (4 bits) becomes one base-w digit
    fn base_w_encode(&self, digest: &[u8; 32]) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.params.l1);

        match self.params.w {
            16 => {
                // w=16: 2 digits per byte (high nibble, low nibble)
                for byte in digest {
                    result.push((byte >> 4) as usize);
                    result.push((byte & 0x0F) as usize);
                }
            }
            256 => {
                // w=256: 1 digit per byte
                for byte in digest {
                    result.push(*byte as usize);
                }
            }
            _ => {
                // Generic case (less efficient)
                let bits_per_digit = (self.params.w as f64).log2() as usize;
                let mut bit_buffer: u64 = 0;
                let mut bits_in_buffer = 0;

                for byte in digest {
                    bit_buffer = (bit_buffer << 8) | (*byte as u64);
                    bits_in_buffer += 8;

                    while bits_in_buffer >= bits_per_digit && result.len() < self.params.l1 {
                        bits_in_buffer -= bits_per_digit;
                        result.push(((bit_buffer >> bits_in_buffer) & ((1 << bits_per_digit) - 1)) as usize);
                    }
                }
            }
        }

        result.truncate(self.params.l1);
        result
    }

    /// Compute the checksum over message indices
    ///
    /// The checksum prevents an attacker from forging signatures for messages
    /// with larger digit values (which would require computing hash preimages).
    fn compute_checksum(&self, msg_indices: &[usize]) -> usize {
        let max_digit = self.params.w - 1;
        msg_indices.iter().map(|&d| max_digit - d).sum()
    }

    /// Encode the checksum as base-w digits
    fn encode_checksum(&self, checksum: usize) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.params.l2);
        let mut remaining = checksum;

        for _ in 0..self.params.l2 {
            result.push(remaining % self.params.w);
            remaining /= self.params.w;
        }

        result.reverse();
        result
    }
}

impl WotsPublicKey {
    /// Compute a short hash of the public key (for address derivation)
    pub fn hash(&self) -> [u8; 32] {
        hash(&self.data)
    }

    /// Get the size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let keypair = WotsKeypair::generate();
        assert!(!keypair.is_used());
        assert_eq!(keypair.public_key_size(), 2144);
    }

    #[test]
    fn test_base_w_encode() {
        let keypair = WotsKeypair::generate();
        let digest = hash(b"test message");
        let encoded = keypair.base_w_encode(&digest);

        // w=16: 64 digits (2 per byte * 32 bytes)
        assert_eq!(encoded.len(), 64);

        // All digits should be in range [0, w-1]
        for digit in &encoded {
            assert!(*digit < keypair.params.w);
        }
    }

    #[test]
    fn test_checksum_decreases_with_larger_message() {
        let keypair = WotsKeypair::generate();

        // All zeros (minimum message)
        let min_msg = [0u8; 32];
        let min_indices = keypair.base_w_encode(&min_msg);
        let min_checksum = keypair.compute_checksum(&min_indices);

        // All 0xFF (maximum message)
        let max_msg = [0xFFu8; 32];
        let max_indices = keypair.base_w_encode(&max_msg);
        let max_checksum = keypair.compute_checksum(&max_indices);

        // Checksum DECREASES as message values INCREASE
        assert!(min_checksum > max_checksum);
    }
}
