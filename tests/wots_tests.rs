//! WOTS+ Integration Tests

use post_quantum_cryptography::wots::{WotsKeypair, WotsError};

/// Test that keypair generation produces valid keys
#[test]
fn test_keypair_generation() {
    let keypair = WotsKeypair::generate();

    // Should not be used initially
    assert!(!keypair.is_used());

    // Public key should be the expected size (67 chains * 32 bytes)
    assert_eq!(keypair.public_key_size(), 2144);
}

/// Test the complete sign-and-verify flow
#[test]
fn test_sign_and_verify() {
    let mut keypair = WotsKeypair::generate();
    let public_key = keypair.public_key_bytes();

    // Create a message digest
    let message = b"Test message for WOTS+ signing";
    let digest = blake3::hash(message).into();

    // Sign the message
    let signature = keypair.sign(&digest).expect("Signing should succeed");

    // Verify the signature
    assert!(
        signature.verify(&public_key, &digest).is_ok(),
        "Signature should verify"
    );

    // Keypair should now be marked as used
    assert!(keypair.is_used());
}

/// Test that the one-time property is enforced
#[test]
fn test_one_time_property() {
    let mut keypair = WotsKeypair::generate();

    let digest1: [u8; 32] = blake3::hash(b"First message").into();
    let digest2: [u8; 32] = blake3::hash(b"Second message").into();

    // First signature should succeed
    let sig1 = keypair.sign(&digest1);
    assert!(sig1.is_ok(), "First signature should succeed");

    // Second signature should fail
    let sig2 = keypair.sign(&digest2);
    assert!(
        matches!(sig2, Err(WotsError::KeyAlreadyUsed)),
        "Second signature should fail with KeyAlreadyUsed"
    );
}

/// Test that verification fails for wrong message
#[test]
fn test_wrong_message_verification_fails() {
    let mut keypair = WotsKeypair::generate();
    let public_key = keypair.public_key_bytes();

    let correct_digest: [u8; 32] = blake3::hash(b"Correct message").into();
    let wrong_digest: [u8; 32] = blake3::hash(b"Wrong message").into();

    let signature = keypair.sign(&correct_digest).unwrap();

    // Should fail with wrong message
    assert!(
        signature.verify(&public_key, &wrong_digest).is_err(),
        "Verification should fail for wrong message"
    );
}

/// Test that verification fails for wrong public key
#[test]
fn test_wrong_key_verification_fails() {
    let mut keypair1 = WotsKeypair::generate();
    let keypair2 = WotsKeypair::generate();

    let digest: [u8; 32] = blake3::hash(b"Test message").into();

    // Sign with keypair1
    let signature = keypair1.sign(&digest).unwrap();

    // Try to verify with keypair2's public key
    let wrong_pk = keypair2.public_key_bytes();
    assert!(
        signature.verify(&wrong_pk, &digest).is_err(),
        "Verification should fail with wrong public key"
    );
}

/// Test signature serialization roundtrip
#[test]
fn test_signature_serialization() {
    let mut keypair = WotsKeypair::generate();
    let public_key = keypair.public_key_bytes();

    let digest: [u8; 32] = blake3::hash(b"Serialize me").into();
    let signature = keypair.sign(&digest).unwrap();

    // Serialize
    let bytes = signature.to_bytes();
    assert!(!bytes.is_empty());

    // Deserialize
    let recovered =
        post_quantum_cryptography::wots::WotsSignature::from_bytes(&bytes).expect("Should deserialize");

    // Should still verify
    assert!(
        recovered.verify(&public_key, &digest).is_ok(),
        "Recovered signature should verify"
    );
}

/// Test that different keypairs produce different signatures for the same message
#[test]
fn test_different_keypairs_different_signatures() {
    let mut keypair1 = WotsKeypair::generate();
    let mut keypair2 = WotsKeypair::generate();

    let digest: [u8; 32] = blake3::hash(b"Same message").into();

    let sig1 = keypair1.sign(&digest).unwrap();
    let sig2 = keypair2.sign(&digest).unwrap();

    // Signatures should be different (different private keys)
    assert_ne!(
        sig1.to_bytes(),
        sig2.to_bytes(),
        "Different keypairs should produce different signatures"
    );
}

/// Test signature size matches expectations
#[test]
fn test_signature_size() {
    let mut keypair = WotsKeypair::generate();
    let digest: [u8; 32] = blake3::hash(b"Test").into();

    let signature = keypair.sign(&digest).unwrap();

    // 67 chains * 32 bytes = 2144 bytes
    assert_eq!(signature.size_bytes(), 2144);
}

/// Verify many random sign/verify cycles
#[test]
fn test_multiple_keypairs_sign_verify() {
    for i in 0..10 {
        let mut keypair = WotsKeypair::generate();
        let public_key = keypair.public_key_bytes();

        let message = format!("Test message {}", i);
        let digest: [u8; 32] = blake3::hash(message.as_bytes()).into();

        let signature = keypair.sign(&digest).expect("Should sign");
        assert!(
            signature.verify(&public_key, &digest).is_ok(),
            "Should verify for iteration {}",
            i
        );
    }
}
