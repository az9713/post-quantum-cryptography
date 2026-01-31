//! Quantum Winter PoC - Post-Quantum Cryptography for Blockchain
//!
//! This library demonstrates post-quantum cryptographic solutions including:
//! - WOTS+ (Winternitz One-Time Signatures Plus) for quantum-safe signatures
//! - Blockchain transaction simulation with post-quantum signatures
//! - Educational demonstrations of quantum threats to current cryptography

pub mod wots;
pub mod blockchain;
pub mod demo;

pub use wots::{WotsKeypair, WotsSignature, WotsError};
pub use blockchain::{Transaction, Address};
