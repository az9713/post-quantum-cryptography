//! Blockchain Simulation Module
//!
//! This module simulates blockchain transactions using WOTS+ signatures
//! to demonstrate how post-quantum cryptography would work in practice.
//!
//! # Key Differences from ECDSA-based Blockchains
//!
//! 1. **Address Reuse**: In Bitcoin/Ethereum, addresses can receive multiple times.
//!    With WOTS+, each keypair should only be used once. This requires:
//!    - Fresh keypair for each transaction's change output
//!    - Or hierarchical deterministic key derivation
//!
//! 2. **Transaction Size**: WOTS+ signatures are ~33x larger than ECDSA.
//!    Block sizes would need adjustment or STARK aggregation.
//!
//! 3. **Key Management**: Users need to generate many more keys,
//!    making HD wallet schemes even more important.

mod transaction;
mod address;

pub use transaction::Transaction;
pub use address::Address;

use thiserror::Error;

/// Errors that can occur during blockchain operations
#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Transaction signature missing")]
    MissingSignature,

    #[error("Transaction signature verification failed")]
    InvalidSignature,

    #[error("WOTS error: {0}")]
    WotsError(#[from] crate::wots::WotsError),

    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: u64, need: u64 },

    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
}
