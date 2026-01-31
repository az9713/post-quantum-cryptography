//! Educational Output and Explanations
//!
//! This module provides colorful, educational explanations of
//! post-quantum cryptography concepts.

use colored::*;

/// Print a section header
pub fn print_header(title: &str) {
    println!();
    println!("{}", "=".repeat(70).bright_blue());
    println!("{}", title.bright_white().bold());
    println!("{}", "=".repeat(70).bright_blue());
    println!();
}

/// Print a subsection header
pub fn print_subheader(title: &str) {
    println!();
    println!("{}", format!("--- {} ---", title).cyan());
    println!();
}

/// Print an informational message
pub fn print_info(message: &str) {
    println!("{} {}", "[INFO]".blue(), message);
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "[OK]".green(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "[WARN]".yellow(), message);
}

/// Print an error message
pub fn print_error(message: &str) {
    println!("{} {}", "[ERROR]".red(), message);
}

/// Print educational content in a box
pub fn print_education_box(title: &str, content: &str) {
    let width = 68;
    let border = "+".bright_magenta();
    let dash = "-".repeat(width).bright_magenta();

    println!();
    println!("{}{}{}", border, dash, border);
    println!(
        "{} {:<width$} {}",
        border,
        title.bright_yellow().bold(),
        border,
        width = width
    );
    println!("{}{}{}", border, dash, border);

    for line in content.lines() {
        if line.len() <= width {
            println!("{} {:<width$} {}", border, line, border, width = width);
        } else {
            // Word wrap long lines
            let mut current_line = String::new();
            for word in line.split_whitespace() {
                if current_line.len() + word.len() + 1 > width {
                    println!(
                        "{} {:<width$} {}",
                        border,
                        current_line,
                        border,
                        width = width
                    );
                    current_line = word.to_string();
                } else {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
            if !current_line.is_empty() {
                println!(
                    "{} {:<width$} {}",
                    border,
                    current_line,
                    border,
                    width = width
                );
            }
        }
    }

    println!("{}{}{}", border, dash, border);
    println!();
}

/// Explain the quantum threat
pub fn explain_quantum_threat() {
    print_education_box(
        "THE QUANTUM THREAT TO BLOCKCHAIN",
        "\
Today's blockchains (Bitcoin, Ethereum) use ECDSA signatures based on
the Elliptic Curve Discrete Logarithm Problem (ECDLP).

Shor's Algorithm (1994) can solve ECDLP in polynomial time on a
quantum computer. This means:

  * Given a public key, derive the private key
  * Forge signatures for any transaction
  * Steal funds from any exposed address

Timeline estimates for 'Q-Day' (when quantum computers can break
ECDSA):
  * Conservative: 2035-2040
  * Optimistic:   2030
  * Industry:     Planning NOW (Harvest Now, Decrypt Later)

~$718 billion in Bitcoin sits in quantum-vulnerable addresses where
public keys have been exposed through previous transactions.",
    );
}

/// Explain WOTS+ (Winternitz One-Time Signatures)
pub fn explain_wots() {
    print_education_box(
        "WOTS+ (Winternitz One-Time Signatures Plus)",
        "\
WOTS+ is a hash-based signature scheme that remains secure against
quantum computers because it only relies on hash function security.

HOW IT WORKS:

1. KEY GENERATION:
   * Generate 67 random 32-byte seeds (private key)
   * Hash each seed 15 times to get public key endpoints
   * Public key = all 67 endpoints concatenated

2. SIGNING:
   * Convert message hash to base-16 digits (+ checksum)
   * For each digit d, reveal the seed hashed d times
   * Verifier can hash forward to reach public key

3. WHY QUANTUM-SAFE:
   * Security = finding hash pre-images
   * Grover's algorithm: only sqrt(N) speedup
   * With 256-bit hashes: still 128-bit post-quantum security

4. THE CATCH - ONE-TIME ONLY:
   * Each signature reveals part of the hash chains
   * Two signatures = enough info to forge new signatures
   * Must use fresh keypair for every transaction!",
    );
}

/// Explain the checksum mechanism
pub fn explain_checksum() {
    print_education_box(
        "THE CHECKSUM: Preventing Signature Forgery",
        "\
Without a checksum, WOTS+ would be vulnerable to a simple attack:

ATTACK SCENARIO (no checksum):
* Signature reveals H^m(seed) for message digit m
* Attacker can hash this forward to get H^(m+1), H^(m+2), etc.
* This lets them forge signatures for LARGER message values!

THE CHECKSUM SOLUTION:
* Checksum C = sum of (w-1 - m_i) for all message digits
* As message digits INCREASE, checksum DECREASES
* To forge a larger message, attacker needs SMALLER checksum
* But smaller checksum requires computing hash PRE-IMAGES
* Pre-images are computationally infeasible to find!

Example (w=16):
* Message 0x00: digit=0, checksum contribution = 15
* Message 0xFF: digit=15, checksum contribution = 0
* Forging 0xFF from 0x00 signature would need to DECREASE checksum",
    );
}

/// Explain STARK aggregation
pub fn explain_stark_aggregation() {
    print_education_box(
        "STARK AGGREGATION: Solving the Size Problem",
        "\
WOTS+ signatures are ~33x larger than ECDSA (2144 vs 64 bytes).
For a block with 1000 transactions:
  * ECDSA: ~64 KB of signatures
  * WOTS+: ~2.1 MB of signatures

STARKs (Scalable Transparent ARguments of Knowledge) solve this:

HOW IT WORKS:
1. Prover verifies all 1000 WOTS+ signatures off-chain
2. Generates ONE proof that 'I verified all signatures correctly'
3. Proof size: ~50 KB regardless of transaction count
4. Verifier checks the proof in O(log n) time

PROPERTIES:
  * Transparent: No trusted setup (unlike SNARKs)
  * Post-quantum: Based on hash functions
  * Scalable: Proof size grows logarithmically

This is why Ethereum Foundation is pursuing 'LeanVM' - making the
EVM a thin wrapper around a STARK verifier.",
    );
}

/// Explain the Ethereum Foundation response
pub fn explain_ef_response() {
    print_education_box(
        "ETHEREUM FOUNDATION RESPONSE (January 2026)",
        "\
The Ethereum Foundation declared post-quantum security a 'top
strategic priority' in January 2026:

INITIATIVES:
  * New Post-Quantum team led by Thomas Coratger
  * $1M Poseidon Prize for hash function strengthening
  * $1M Proximity Prize for PQ cryptography advances
  * Biweekly developer sessions on PQ transactions
  * Multi-client post-quantum consensus devnets running

THE LEANVM STRATEGY:
  * Minimalist zkVM design
  * EVM becomes a thin wrapper around STARK verifier
  * Enables replacing ECDSA with hash-based signatures
  * Backward-compatible upgrade path via account abstraction

TIMELINE:
  * 2026: First wave of PQ compliance requirements
  * 2030: US/EU require critical infrastructure transition
  * 10+ years: Full decentralized network integration",
    );
}

/// Print a visual signature comparison
pub fn print_signature_comparison() {
    println!();
    println!("{}", "SIGNATURE SIZE COMPARISON".bright_white().bold());
    println!();

    let ecdsa_size = 64;
    let wots_size = 2144;
    let scale = 50; // Characters per 2144 bytes

    let ecdsa_bar = "█".repeat((ecdsa_size as f64 / wots_size as f64 * scale as f64) as usize);
    let wots_bar = "█".repeat(scale);

    println!("  ECDSA ({:>4} bytes): {}", ecdsa_size, ecdsa_bar.green());
    println!("  WOTS+ ({:>4} bytes): {}", wots_size, wots_bar.yellow());
    println!();
    println!(
        "  WOTS+ is {}x larger - this is the quantum resistance trade-off.",
        format!("~{:.0}", wots_size as f64 / ecdsa_size as f64).red()
    );
    println!();
}

/// Print demo introduction
pub fn print_intro() {
    println!();
    println!(
        "{}",
        r#"
   ____                   _                    __        ___       _
  / __ \__  ______ _____ | |_ __  ______ ___  / /       / (_)___  / /____  _____
 / / / / / / / __ `/ __ \| __/ / / / __ `__ \/ / | /| / / / __ \/ __/ _ \/ ___/
/ /_/ / /_/ / /_/ / / / / /_/ /_/ / / / / / /_/| |/ |/ / / / / / /_/  __/ /
\___\_\__,_/\__,_/_/ /_/\__/\__,_/_/ /_/ /_(_) |__/|__/_/_/ /_/\__/\___/_/

"#
        .bright_cyan()
    );
    println!(
        "{}",
        "Post-Quantum Cryptography Proof of Concept".bright_white()
    );
    println!("{}", "Based on a16z crypto research and Ethereum Foundation initiatives".dimmed());
    println!();
}

/// Print demo conclusion
pub fn print_conclusion() {
    print_education_box(
        "KEY TAKEAWAYS",
        "\
1. ECDSA (Bitcoin/Ethereum) will be broken by quantum computers
   - Shor's algorithm solves discrete log in polynomial time
   - Q-Day estimated 2030-2040, but 'Harvest Now, Decrypt Later'

2. WOTS+ provides quantum-resistant signatures
   - Security relies only on hash function pre-image resistance
   - Trade-off: ~33x larger signatures, one-time use only

3. STARK aggregation solves the size problem
   - Compress thousands of signatures into one ~50KB proof
   - Also post-quantum (based on hashes)

4. Industry is responding NOW
   - Ethereum Foundation: post-quantum team, $2M in prizes
   - NIST: finalized PQ standards (FIPS 203/204/205) in 2024
   - Regulatory deadlines approaching (2026-2030)

The cryptographic winter is coming. The time to prepare is now.",
    );
}
