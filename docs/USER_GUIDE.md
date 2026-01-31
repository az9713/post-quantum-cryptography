# User Guide

This guide explains everything you need to know to use this application effectively. No prior experience with Rust or cryptography is required.

> **Note:** This is a **cryptography demonstration** with educational focus. The WOTS+ signatures are production-quality, but the "blockchain" is a minimal simulation (no actual blocks, consensus, or networking).

---

## Table of Contents

1. [What Is This Application?](#what-is-this-application)
2. [Prerequisites](#prerequisites)
3. [Installation Step-by-Step](#installation-step-by-step)
4. [Running the Application](#running-the-application)
5. [Understanding the Output](#understanding-the-output)
6. [Running Tests](#running-tests)
7. [Glossary of Terms](#glossary-of-terms)
8. [Frequently Asked Questions](#frequently-asked-questions)
9. [Troubleshooting](#troubleshooting)

---

## What Is This Application?

### The Problem It Solves

Bitcoin, Ethereum, and most cryptocurrencies use a signature algorithm called **ECDSA**. This algorithm will be **broken by quantum computers** using something called Shor's Algorithm.

**When will this happen?**
- Conservative estimate: 2035-2040
- Optimistic estimate: 2030
- Current threat: "Harvest Now, Decrypt Later" - attackers storing encrypted data today

**What's at risk?**
- ~$718 billion in Bitcoin in vulnerable addresses
- All Ethereum accounts that have ever sent a transaction

### The Solution This Application Demonstrates

This application implements **WOTS+** (Winternitz One-Time Signatures Plus), a signature scheme that:
- Remains secure against quantum computers
- Only relies on hash functions (which quantum computers can't easily break)
- Is already being considered by the Ethereum Foundation

### What This Application Does

1. **Educates** - Explains the quantum threat with colorful visualizations
2. **Demonstrates** - Shows WOTS+ key generation, signing, and verification
3. **Proves** - Includes 48 tests verifying all security properties

---

## Prerequisites

### What You Need

| Requirement | Why | How to Get It |
|-------------|-----|---------------|
| Computer | Runs the application | Windows, Mac, or Linux |
| Terminal | Enter commands | Built into all operating systems |
| Rust | Programming language | Free download (instructions below) |
| ~500MB disk space | For Rust and dependencies | - |

### What You DON'T Need

- Programming experience (helpful but not required)
- Cryptography knowledge (the app explains everything)
- Special hardware (runs on any modern computer)

---

## Installation Step-by-Step

### Step 1: Open a Terminal

**Windows:**
1. Press the Windows key
2. Type "PowerShell" or "Git Bash"
3. Press Enter

**Mac:**
1. Press Cmd + Space
2. Type "Terminal"
3. Press Enter

**Linux:**
1. Press Ctrl + Alt + T

### Step 2: Check if Rust is Installed

Type this command and press Enter:
```bash
rustc --version
```

**If you see a version number** (like `rustc 1.93.0`):
- Rust is already installed
- Skip to Step 4

**If you see "command not found":**
- Continue to Step 3

### Step 3: Install Rust

**Windows (recommended method):**
```
winget install Rustlang.Rustup
```

**Windows (alternative):**
1. Go to https://rustup.rs/
2. Download rustup-init.exe
3. Run the installer
4. Press Enter to accept defaults

**Mac/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then press Enter to accept defaults.

### Step 4: Restart Your Terminal

**Important!** Close your terminal completely and open a new one. This ensures Rust is in your PATH.

### Step 5: Verify Rust Installation

```bash
cargo --version
```

You should see: `cargo 1.93.0` (or similar version)

### Step 6: Clone and Navigate to the Project

```bash
# Clone the repository
git clone https://github.com/az9713/post-quantum-cryptography.git

# Navigate into the project
cd post-quantum-cryptography
```

### Step 7: Build the Project (First Time Only)

```bash
cargo build
```

This downloads dependencies and compiles the code. Takes 1-2 minutes the first time.

**Expected output:**
```
   Compiling quantum-winter-poc v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 45.23s
```

---

## Running the Application

### The Main Demo

```bash
cargo run
```

This runs the educational demonstration showing:
- Quantum threat explanation
- WOTS+ key generation
- Transaction signing
- One-time property enforcement
- Size comparisons

### What to Expect

The output is colorful and educational. It takes about 1 second to run and displays ~150 lines of information.

---

## Understanding the Output

### Section 1: ASCII Art Banner

```
   ____                   _                    __        ___       _
  / __ \__  ______ _____ | |_ __  ______ ___  / /       / (_)___  / /____  _____
  ...
```

This is just a decorative title.

### Section 2: The Quantum Threat

```
+--------------------------------------------------------------------+
| THE QUANTUM THREAT TO BLOCKCHAIN                                    |
+--------------------------------------------------------------------+
```

This explains:
- Why current cryptography (ECDSA) is vulnerable
- Shor's Algorithm and what it does
- Timeline estimates for "Q-Day"
- How much money is at risk

### Section 3: WOTS+ Explanation

```
+--------------------------------------------------------------------+
| WOTS+ (Winternitz One-Time Signatures Plus)                         |
+--------------------------------------------------------------------+
```

This explains:
- How WOTS+ works (hash chains)
- Why it's quantum-safe
- The one-time limitation

### Section 4: Demo 1 - Key Generation

```
======================================================================
DEMO 1: WOTS+ Key Generation
======================================================================

[OK] Keypair generated in 2.01ms
```

This shows:
- **Timing**: How long key generation takes
- **Parameters**: Winternitz parameter (w=16), security level (256 bits)
- **Sizes**: Public key is 2144 bytes (vs 33 bytes for ECDSA)
- **Address**: A sample blockchain address starting with "qw1"

### Section 5: Demo 2 - Transaction Signing

```
======================================================================
DEMO 2: Transaction Signing with WOTS+
======================================================================

[OK] Transaction signed in 234μs
[OK] Signature verified in 187μs
```

This shows:
- Creating a transaction between two addresses
- Signing the transaction
- Verifying the signature
- The signature size (2144 bytes)

### Section 6: Demo 3 - One-Time Property

```
======================================================================
DEMO 3: One-Time Property Enforcement
======================================================================

[OK] First signature succeeded (2144 bytes)
[OK] Second signature correctly blocked: 'Key has already been used'
```

This demonstrates the critical security feature:
- First signature works
- Second signature is **blocked**
- This prevents signature forgery

### Section 7: Size Comparison

```
SIGNATURE SIZE COMPARISON

  ECDSA (  64 bytes): █
  WOTS+ (2144 bytes): ██████████████████████████████████████████████████

  WOTS+ is ~34x larger
```

A visual comparison showing WOTS+ signatures are much larger than ECDSA.

### Section 8: Additional Education

More boxes explaining:
- **The Checksum**: How forgery is prevented
- **STARK Aggregation**: How to solve the size problem
- **Ethereum Foundation Response**: Industry initiatives

### Section 9: Key Takeaways

```
+--------------------------------------------------------------------+
| KEY TAKEAWAYS                                                       |
+--------------------------------------------------------------------+
```

Summary of everything learned.

---

## Running Tests

### Run All Tests

```bash
cargo test
```

**Expected output:**
```
running 48 tests
...
test result: ok. 48 passed; 0 failed
```

### Run Specific Test Categories

**WOTS+ cryptography tests (9 tests):**
```bash
cargo test --test wots_tests
```

**Blockchain transaction tests (14 tests):**
```bash
cargo test --test blockchain_tests
```

**Library unit tests (24 tests):**
```bash
cargo test --lib
```

### Run a Single Test

```bash
cargo test test_one_time_property
```

### Understanding Test Output

- `ok` means the test passed
- `FAILED` means the test found a bug (you shouldn't see this)
- The final line shows total passed/failed

---

## Glossary of Terms

### Cryptography Terms

| Term | Simple Explanation |
|------|-------------------|
| **ECDSA** | The signature algorithm Bitcoin/Ethereum use today. Will be broken by quantum computers. |
| **WOTS+** | Winternitz One-Time Signatures Plus. Quantum-safe signature scheme. |
| **Hash** | A function that turns any input into a fixed-size output. Like a fingerprint for data. |
| **Hash Chain** | Hashing a value multiple times: H(H(H(x))). Central to WOTS+. |
| **Private Key** | Secret value only you know. Used to sign transactions. |
| **Public Key** | Derived from private key. Can be shared. Used to verify signatures. |
| **Signature** | Proof that the private key holder authorized something. |
| **Digest** | The hash of a message. What actually gets signed. |

### Quantum Terms

| Term | Simple Explanation |
|------|-------------------|
| **Quantum Computer** | A computer using quantum physics. Can solve certain problems exponentially faster. |
| **Shor's Algorithm** | Quantum algorithm that breaks ECDSA, RSA, and similar cryptography. |
| **Grover's Algorithm** | Quantum algorithm that speeds up searching. Only provides square root speedup (not a big threat). |
| **Q-Day** | The day quantum computers become powerful enough to break current cryptography. |
| **Post-Quantum** | Cryptography designed to resist quantum attacks. |

### Blockchain Terms

| Term | Simple Explanation |
|------|-------------------|
| **Transaction** | A transfer of value (like sending Bitcoin). |
| **Address** | Like a bank account number. Where you receive funds. |
| **Nonce** | A counter preventing replay attacks. |
| **STARK** | A proof system that can compress many operations into one small proof. |

### Rust/Programming Terms

| Term | Simple Explanation |
|------|-------------------|
| **Cargo** | Rust's build tool and package manager. Like npm for JavaScript. |
| **Crate** | A Rust package/library. |
| **Compile** | Converting source code into an executable program. |
| **Build** | Same as compile. |
| **Test** | Automated verification that code works correctly. |

---

## Frequently Asked Questions

### General Questions

**Q: Do I need to understand cryptography to use this?**
A: No. The application explains everything with colorful, educational output.

**Q: Is this production-ready?**
A: No. This is an educational proof-of-concept. Do not use for real money.

**Q: Why is WOTS+ called "one-time"?**
A: Each keypair can only safely sign ONE message. Signing twice reveals information that enables forgery.

**Q: Why are WOTS+ signatures so large?**
A: WOTS+ uses 67 hash chains × 32 bytes = 2144 bytes. This is the trade-off for quantum resistance.

### Technical Questions

**Q: Why Blake3 instead of SHA-256?**
A: Blake3 is faster and still quantum-resistant. Both would work.

**Q: Why not use a library like `winternitz-ots`?**
A: Educational value. Implementing from scratch helps understand the algorithm.

**Q: Can WOTS+ be used for real blockchain?**
A: Yes, but you'd need STARK aggregation to handle the signature size. The Ethereum Foundation is working on this.

### Installation Questions

**Q: Why do I need Rust?**
A: This application is written in Rust. Rust compiles to native code for best performance.

**Q: Can I use this without installing Rust?**
A: Not currently. You could build a release and share the binary, but compilation requires Rust.

**Q: The installation seems stuck. What do I do?**
A: First-time compilation downloads dependencies. It can take 1-5 minutes depending on internet speed.

---

## Troubleshooting

### "cargo: command not found"

**Cause:** Rust isn't installed or not in PATH.

**Solution 1 (Git Bash):**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Solution 2:** Restart your terminal after installing Rust.

**Solution 3:** Reinstall Rust:
```bash
# Windows
winget install Rustlang.Rustup

# Mac/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "error: could not find `Cargo.toml`"

**Cause:** You're in the wrong directory.

**Solution:** Navigate to the project:
```bash
cd /path/to/quantum-winter-poc
```

Verify you're in the right place:
```bash
ls Cargo.toml
```

### "error[E0433]: failed to resolve"

**Cause:** Dependencies not downloaded.

**Solution:**
```bash
cargo build
```

### Tests fail

**Cause:** Likely a code modification broke something.

**Solution:**
1. Make sure you haven't modified any files
2. Try `cargo clean && cargo build`

### Colors don't show

**Cause:** Terminal doesn't support ANSI colors.

**Solution:** Use Git Bash on Windows, or any modern terminal on Mac/Linux.

### First run is very slow

**Cause:** First compilation downloads and builds all dependencies.

**Expected time:** 1-5 minutes.

**Future runs:** Nearly instant (everything is cached).

---

## Sample Outputs

Can't run the application yourself? Pre-captured outputs are available:

- **[Demo Output](examples/demo_results.txt)** - Complete `cargo run` output with educational content
- **[Test Results](examples/test_results.txt)** - Full `cargo test` output showing all 48 passing tests

---

## Next Steps

Now that you understand how to use the application:

1. **Explore the Quick Start Guide** - [QUICK_START.md](QUICK_START.md) for 10 hands-on examples
2. **Read the Architecture** - [ARCHITECTURE.md](ARCHITECTURE.md) to understand how it works
3. **Become a Developer** - [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) to modify and extend the code
4. **Learn the Theory** - [Tutorials](../tutorials/00-INDEX.md) for comprehensive learning from beginner to PhD level
