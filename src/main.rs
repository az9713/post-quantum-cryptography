//! Post-Quantum Cryptography Demo
//!
//! This demo showcases WOTS+ post-quantum signatures for blockchain security.

use colored::Colorize;
use post_quantum_cryptography::{
    blockchain::{Address, Transaction},
    demo::*,
    wots::{WotsKeypair, WotsParams, WotsError},
};

fn main() {
    // Print introduction
    print_intro();

    // Educational sections
    explain_quantum_threat();
    explain_wots();

    // Demo 1: Key Generation
    demo_key_generation();

    // Demo 2: Transaction Signing
    demo_transaction_signing();

    // Demo 3: One-Time Property
    demo_one_time_property();

    // Demo 4: Signature Comparison
    print_signature_comparison();

    // More educational content
    explain_checksum();
    explain_stark_aggregation();
    explain_ef_response();

    // Conclusion
    print_conclusion();
}

/// Demonstrate WOTS+ key generation
fn demo_key_generation() {
    print_header("DEMO 1: WOTS+ Key Generation");

    print_info("Generating a new WOTS+ keypair...");
    println!();

    let (keypair, duration) = time_operation(|| WotsKeypair::generate());

    let params = WotsParams::default();
    let address = Address::from_keypair(&keypair);

    print_success(&format!("Keypair generated in {}", format_duration(duration)));
    println!();

    println!("  {} Parameters:", "WOTS+".cyan());
    println!("    - Winternitz parameter (w): {}", params.w);
    println!("    - Security level (n):       {} bits", params.n * 8);
    println!("    - Message chains (l1):      {}", params.l1);
    println!("    - Checksum chains (l2):     {}", params.l2);
    println!("    - Total chains:             {}", params.total_chains());
    println!();

    println!("  {} Sizes:", "Key".cyan());
    println!(
        "    - Private key:  {} bytes ({} random seeds)",
        params.total_chains() * 32,
        params.total_chains()
    );
    println!(
        "    - Public key:   {} bytes ({} chain endpoints)",
        keypair.public_key_size(),
        params.total_chains()
    );
    println!();

    println!("  {} Derived address:", "Blockchain".cyan());
    println!("    {}", address.to_string().green());
    println!("    (short: {})", address.short_display());
    println!();

    print_info("Compare to ECDSA (Bitcoin/Ethereum):");
    println!("    - ECDSA private key: 32 bytes");
    println!("    - ECDSA public key:  33 bytes (compressed)");
    println!(
        "    - WOTS+ public key is {}x larger!",
        format!("~{:.0}", keypair.public_key_size() as f64 / 33.0).yellow()
    );
}

/// Demonstrate transaction signing
fn demo_transaction_signing() {
    print_header("DEMO 2: Transaction Signing with WOTS+");

    // Generate keypairs for sender and recipient
    print_info("Generating keypairs for sender and recipient...");

    let mut sender_keypair = WotsKeypair::generate();
    let recipient_keypair = WotsKeypair::generate();

    let sender_addr = Address::from_keypair(&sender_keypair);
    let recipient_addr = Address::from_keypair(&recipient_keypair);

    println!("  Sender:    {}", sender_addr.short_display().cyan());
    println!("  Recipient: {}", recipient_addr.short_display().green());
    println!();

    // Create transaction
    print_info("Creating transaction: 1000 units from sender to recipient...");

    let mut tx = Transaction::new(sender_addr.clone(), recipient_addr.clone(), 1000, 0);

    println!("  Transaction digest: 0x{}...", hex::encode(&tx.compute_digest()[..8]));
    println!();

    // Sign transaction
    print_info("Signing transaction with WOTS+ keypair...");

    let (sign_result, sign_duration) = time_operation(|| tx.sign(&mut sender_keypair));
    sign_result.expect("Signing should succeed");

    print_success(&format!("Transaction signed in {}", format_duration(sign_duration)));
    println!();

    println!("  Signature size: {} bytes", tx.signature_size().unwrap());
    println!("  Transaction size: {} bytes (including signature + pubkey)", tx.size_bytes());
    println!();

    // Verify transaction
    print_info("Verifying transaction signature...");

    let (verify_result, verify_duration) = time_operation(|| tx.verify_signature());
    verify_result.expect("Verification should succeed");

    print_success(&format!(
        "Signature verified in {}",
        format_duration(verify_duration)
    ));
    println!();

    // Show that keypair is now used
    if sender_keypair.is_used() {
        print_warning("Sender keypair is now marked as USED - cannot sign again!");
    }
}

/// Demonstrate the one-time property
fn demo_one_time_property() {
    print_header("DEMO 3: One-Time Property Enforcement");

    print_education_box(
        "Why One-Time Matters",
        "\
In WOTS+, each signature reveals part of the hash chains. If you
sign TWO messages with the same key:

  * First signature reveals: H^m1(seed) for each chain
  * Second signature reveals: H^m2(seed) for each chain

An attacker who sees BOTH signatures can:
  * Combine revealed chain values
  * Forge signatures for any message
  * Steal all funds from that address!

This is why our implementation BLOCKS second signatures.",
    );

    // Generate a keypair
    let mut keypair = WotsKeypair::generate();
    let address = Address::from_keypair(&keypair);

    print_info(&format!("Generated address: {}", address.short_display()));
    println!();

    // First signature - should succeed
    let digest1 = [0u8; 32]; // Dummy digest
    print_info("Attempting first signature...");

    match keypair.sign(&digest1) {
        Ok(sig) => {
            print_success(&format!("First signature succeeded ({} bytes)", sig.size_bytes()));
        }
        Err(e) => {
            print_error(&format!("Unexpected error: {}", e));
        }
    }
    println!();

    // Second signature - should fail
    let digest2 = [1u8; 32]; // Different digest
    print_info("Attempting second signature (should fail)...");

    match keypair.sign(&digest2) {
        Ok(_) => {
            print_error("SECURITY BUG: Second signature should have been blocked!");
        }
        Err(WotsError::KeyAlreadyUsed) => {
            print_success("Second signature correctly blocked: 'Key has already been used'");
            println!();
            println!(
                "  {} This protects against the catastrophic 'two-signature' attack.",
                "Security:".green()
            );
            println!(
                "  {} Generate a NEW keypair for the next transaction.",
                "Practice:".cyan()
            );
        }
        Err(e) => {
            print_error(&format!("Unexpected error type: {}", e));
        }
    }
    println!();

    print_info("In a real blockchain:");
    println!("  * Wallets auto-generate fresh keypairs for change outputs");
    println!("  * Address reuse is architecturally prevented");
    println!("  * HD wallet schemes derive unlimited keys from one seed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_runs() {
        // Just verify the demos don't panic
        demo_key_generation();
        demo_transaction_signing();
        demo_one_time_property();
    }
}
