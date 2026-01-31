//! Blockchain Module Integration Tests

use post_quantum_cryptography::{
    blockchain::{Address, Transaction},
    wots::WotsKeypair,
};

/// Test address derivation from keypair
#[test]
fn test_address_from_keypair() {
    let keypair = WotsKeypair::generate();
    let address = Address::from_keypair(&keypair);

    // Address should be deterministic
    let address2 = Address::from_keypair(&keypair);
    assert_eq!(address, address2);

    // Address string should have qw1 prefix
    let addr_string = address.to_string();
    assert!(addr_string.starts_with("qw1"));
}

/// Test address string parsing
#[test]
fn test_address_string_roundtrip() {
    let keypair = WotsKeypair::generate();
    let address = Address::from_keypair(&keypair);

    let string = address.to_string();
    let parsed = Address::from_string(&string).expect("Should parse");

    assert_eq!(address, parsed);
}

/// Test different keypairs produce different addresses
#[test]
fn test_different_keypairs_different_addresses() {
    let keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let addr1 = Address::from_keypair(&keypair1);
    let addr2 = Address::from_keypair(&keypair2);

    assert_ne!(addr1, addr2);
}

/// Test transaction creation
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
    assert_eq!(tx.nonce, 0);
    assert!(!tx.is_signed());
}

/// Test transaction signing
#[test]
fn test_transaction_signing() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let mut tx = Transaction::new(from, to, 1000, 0);

    // Sign should succeed
    assert!(tx.sign(&mut keypair1).is_ok());
    assert!(tx.is_signed());

    // Keypair should be marked as used
    assert!(keypair1.is_used());
}

/// Test transaction verification
#[test]
fn test_transaction_verification() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let mut tx = Transaction::new(from, to, 1000, 0);
    tx.sign(&mut keypair1).unwrap();

    // Verification should succeed
    assert!(tx.verify_signature().is_ok());
}

/// Test tampered transaction fails verification
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

/// Test signing with wrong keypair fails
#[test]
fn test_wrong_keypair_signing_fails() {
    let keypair1 = WotsKeypair::generate();
    let mut keypair2 = WotsKeypair::generate();
    let keypair3 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair3);

    let mut tx = Transaction::new(from, to, 1000, 0);

    // Try to sign with keypair2 (not the sender)
    let result = tx.sign(&mut keypair2);
    assert!(result.is_err());
}

/// Test transaction digest is deterministic
#[test]
fn test_transaction_digest_deterministic() {
    let keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let tx1 = Transaction::new(from.clone(), to.clone(), 1000, 0);
    let tx2 = Transaction::new(from, to, 1000, 0);

    assert_eq!(tx1.compute_digest(), tx2.compute_digest());
}

/// Test transaction JSON serialization roundtrip
#[test]
fn test_transaction_json_roundtrip() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let mut tx = Transaction::new(from, to, 1000, 0);
    tx.sign(&mut keypair1).unwrap();

    // Serialize to JSON
    let json = tx.to_json().expect("Should serialize");

    // Deserialize from JSON
    let recovered = Transaction::from_json(&json).expect("Should deserialize");

    // Should still verify
    assert!(recovered.verify_signature().is_ok());
}

/// Test transaction with data
#[test]
fn test_transaction_with_data() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let data = b"Hello, blockchain!".to_vec();
    let mut tx = Transaction::new(from, to, 1000, 0).with_data(data.clone());

    assert_eq!(tx.data, Some(data));

    // Should sign and verify correctly
    tx.sign(&mut keypair1).unwrap();
    assert!(tx.verify_signature().is_ok());
}

/// Test multiple transactions from different keypairs
#[test]
fn test_multiple_transactions() {
    for i in 0..5 {
        let mut sender = WotsKeypair::generate();
        let recipient = WotsKeypair::generate();

        let from = Address::from_keypair(&sender);
        let to = Address::from_keypair(&recipient);

        let mut tx = Transaction::new(from, to, 1000 * (i + 1), i as u64);
        tx.sign(&mut sender).expect("Should sign");
        assert!(
            tx.verify_signature().is_ok(),
            "Transaction {} should verify",
            i
        );
    }
}

/// Test transaction size is reasonable
#[test]
fn test_transaction_size() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let from = Address::from_keypair(&keypair1);
    let to = Address::from_keypair(&keypair2);

    let mut tx = Transaction::new(from, to, 1000, 0);
    tx.sign(&mut keypair1).unwrap();

    let size = tx.size_bytes();

    // Should be approximately:
    // - from: 32 bytes
    // - to: 32 bytes
    // - amount: 8 bytes
    // - nonce: 8 bytes
    // - signature: ~2200 bytes
    // - pubkey: ~2144 bytes
    assert!(size > 4000, "Transaction should be at least 4KB");
    assert!(size < 6000, "Transaction should be less than 6KB");
}

/// Test zero address
#[test]
fn test_zero_address() {
    let zero = Address::zero();
    assert!(zero.is_zero());

    let keypair = WotsKeypair::generate();
    let regular = Address::from_keypair(&keypair);
    assert!(!regular.is_zero());
}
