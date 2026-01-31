# Developer Guide

This guide provides everything you need to modify, extend, and maintain this codebase. Written for developers with C/C++/Java experience who are new to Rust.

> **Project Scope:** The `wots/` module contains production-quality cryptography. The `blockchain/` module is a minimal transaction simulation for demonstration - not a real blockchain implementation.

---

## Table of Contents

1. [Rust Crash Course for C/C++/Java Developers](#rust-crash-course)
2. [Development Environment Setup](#development-environment-setup)
3. [Project Structure Explained](#project-structure-explained)
4. [Code Walkthrough](#code-walkthrough)
5. [How to Make Changes](#how-to-make-changes)
6. [Adding New Features](#adding-new-features)
7. [Testing Guide](#testing-guide)
8. [Debugging](#debugging)
9. [Common Patterns in This Codebase](#common-patterns)
10. [Extending the Project](#extending-the-project)

---

## Rust Crash Course for C/C++/Java Developers {#rust-crash-course}

### Rust vs C/C++/Java: Quick Comparison

| Concept | C/C++ | Java | Rust |
|---------|-------|------|------|
| Memory management | Manual (malloc/free) | Garbage collected | Ownership system |
| Null pointers | Yes (dangerous) | Yes (NullPointerException) | No (use Option<T>) |
| Error handling | Return codes / exceptions | Exceptions | Result<T, E> |
| Compilation | gcc/g++ | javac → JVM | rustc (via cargo) |
| Package manager | None standard | Maven/Gradle | Cargo |

### Key Rust Concepts

#### 1. Ownership (Instead of Manual Memory or GC)

```rust
// In C++: you'd use new/delete or smart pointers
// In Java: GC handles it
// In Rust: ownership is tracked at compile time

let s1 = String::from("hello");  // s1 owns the string
let s2 = s1;                      // Ownership MOVES to s2
// println!("{}", s1);            // ERROR! s1 no longer valid

let s3 = s2.clone();              // Explicit copy if needed
```

**Why this matters:** No memory leaks, no use-after-free, no data races. Guaranteed at compile time.

#### 2. References and Borrowing

```rust
fn calculate_length(s: &String) -> usize {  // & means "borrow"
    s.len()
}

let s = String::from("hello");
let len = calculate_length(&s);  // s is borrowed, not moved
println!("{} has length {}", s, len);  // s still valid!
```

**Mutable references:**
```rust
fn add_exclamation(s: &mut String) {  // &mut = mutable borrow
    s.push('!');
}

let mut s = String::from("hello");
add_exclamation(&mut s);
println!("{}", s);  // "hello!"
```

#### 3. Option<T> Instead of Null

```rust
// In Java: String name = null;
// In Rust:
let name: Option<String> = None;
let name: Option<String> = Some(String::from("Alice"));

// Must handle both cases:
match name {
    Some(n) => println!("Hello, {}", n),
    None => println!("No name provided"),
}

// Or use if let:
if let Some(n) = name {
    println!("Hello, {}", n);
}
```

#### 4. Result<T, E> Instead of Exceptions

```rust
// In Java: throws Exception
// In Rust: returns Result

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Must handle both cases:
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

// Or use ? to propagate errors:
fn do_math() -> Result<i32, String> {
    let x = divide(10, 2)?;  // Returns early if Err
    let y = divide(x, 5)?;
    Ok(y)
}
```

#### 5. Structs and impl (Like Classes)

```rust
// In Java: class Person { ... }
// In Rust:

struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor (convention: called "new")
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // Method (takes &self)
    fn greet(&self) {
        println!("Hello, I'm {} and I'm {} years old", self.name, self.age);
    }

    // Mutable method (takes &mut self)
    fn have_birthday(&mut self) {
        self.age += 1;
    }
}

// Usage:
let mut alice = Person::new(String::from("Alice"), 30);
alice.greet();
alice.have_birthday();
```

#### 6. Traits (Like Interfaces)

```rust
// In Java: interface Drawable { void draw(); }
// In Rust:

trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}
```

#### 7. Modules (Like Packages)

```rust
// File: src/lib.rs
mod wots;         // Declares module, looks for src/wots.rs or src/wots/mod.rs
mod blockchain;

pub use wots::WotsKeypair;  // Re-export for external use

// File: src/wots/mod.rs
mod keypair;      // Private submodule
mod signature;

pub use keypair::WotsKeypair;  // Make public
```

---

## Development Environment Setup {#development-environment-setup}

### Required Tools

1. **Rust** (includes rustc and cargo)
2. **A code editor** (VS Code recommended)
3. **Git** (for version control)

### Step 1: Install Rust

```bash
# Windows
winget install Rustlang.Rustup

# Mac/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal, then verify:
```bash
rustc --version    # Compiler
cargo --version    # Build tool
```

### Step 2: Install VS Code Extensions (Recommended)

1. Install VS Code: https://code.visualstudio.com/
2. Install extensions:
   - **rust-analyzer** (essential - provides IntelliSense)
   - **Even Better TOML** (for Cargo.toml syntax)
   - **Error Lens** (shows errors inline)

### Step 3: Clone and Build

```bash
cd /path/to/post-quantum-cryptography
cargo build
```

### Step 4: Verify Everything Works

```bash
cargo test
cargo run
```

---

## Project Structure Explained {#project-structure-explained}

```
post-quantum-cryptography/
├── Cargo.toml              # Project configuration (like pom.xml or package.json)
├── Cargo.lock              # Locked dependency versions (auto-generated)
├── src/
│   ├── lib.rs              # Library root - exports public API
│   ├── main.rs             # Binary entry point - runs the demo
│   ├── wots/               # WOTS+ cryptography module
│   │   ├── mod.rs          # Module root - declares submodules
│   │   ├── keypair.rs      # Key generation and signing
│   │   └── signature.rs    # Verification and serialization
│   ├── blockchain/         # Blockchain simulation module
│   │   ├── mod.rs          # Module root
│   │   ├── address.rs      # Address derivation
│   │   └── transaction.rs  # Transaction structures
│   └── demo/               # Educational output module
│       ├── mod.rs          # Module root
│       ├── educational.rs  # Colored explanations
│       └── timing.rs       # Performance measurement
├── tests/                  # Integration tests
│   ├── wots_tests.rs       # WOTS+ tests
│   └── blockchain_tests.rs # Blockchain tests
├── docs/                   # Documentation
│   ├── examples/           # Captured demo and test outputs
│   │   ├── demo_results.txt    # Full cargo run output
│   │   └── test_results.txt    # Full cargo test output
│   └── *.md                # User and developer guides
└── tutorials/              # 11 tutorials (beginner to PhD level)
    └── 00-INDEX.md through 10-PHD-*.md
```

### Understanding Cargo.toml

```toml
[package]
name = "post-quantum-cryptography"    # Crate name
version = "0.1.0"              # Version
edition = "2021"               # Rust edition (language version)

[dependencies]
blake3 = "1.5"                 # Fast hash function
sha3 = "0.10"                  # SHA3 for transaction digests
hex = "0.4"                    # Hex encoding
rand = "0.8"                   # Random number generation
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0"             # JSON serialization
thiserror = "1.0"              # Error type derivation
anyhow = "1.0"                 # Easy error handling
colored = "2.0"                # Colored terminal output

[dev-dependencies]
# Test-only dependencies go here
```

---

## Code Walkthrough {#code-walkthrough}

### Entry Point: src/main.rs

```rust
// Import types from our library
use post_quantum_cryptography::{
    blockchain::{Address, Transaction},
    demo::*,
    wots::{WotsKeypair, WotsParams, WotsError},
};

fn main() {
    // Print educational content
    print_intro();
    explain_quantum_threat();
    explain_wots();

    // Run live demonstrations
    demo_key_generation();
    demo_transaction_signing();
    demo_one_time_property();

    // More education
    print_signature_comparison();
    print_conclusion();
}
```

### Library Root: src/lib.rs

```rust
// Declare modules (tells Rust where to find code)
pub mod wots;
pub mod blockchain;
pub mod demo;

// Re-export commonly used types for convenience
pub use wots::{WotsKeypair, WotsSignature, WotsError};
pub use blockchain::{Transaction, Address};
```

### WOTS+ Module: src/wots/mod.rs

```rust
// Submodules
mod keypair;
mod signature;

// Public exports
pub use keypair::{WotsKeypair, WotsPublicKey};
pub use signature::WotsSignature;

// Configuration parameters
#[derive(Debug, Clone)]
pub struct WotsParams {
    pub w: usize,    // Winternitz parameter (base)
    pub n: usize,    // Security parameter (bytes)
    pub l1: usize,   // Message chains
    pub l2: usize,   // Checksum chains
}

// Error types
#[derive(Error, Debug)]
pub enum WotsError {
    #[error("Key has already been used")]
    KeyAlreadyUsed,

    #[error("Signature verification failed")]
    VerificationFailed,
    // ...
}

// Hash function used throughout
pub fn hash(data: &[u8]) -> [u8; 32] {
    blake3::hash(data).into()
}

// Chain hash: apply hash n times
pub(crate) fn chain_hash(input: &[u8; 32], iterations: usize) -> [u8; 32] {
    let mut result = *input;
    for _ in 0..iterations {
        result = hash(&result);
    }
    result
}
```

### Key Generation: src/wots/keypair.rs

```rust
pub struct WotsKeypair {
    private_key: Vec<[u8; 32]>,  // 67 random seeds
    public_key: Vec<[u8; 32]>,   // 67 chain endpoints
    used: bool,                   // One-time tracking
    params: WotsParams,
}

impl WotsKeypair {
    pub fn generate() -> Self {
        let params = WotsParams::default();  // w=16, n=32, l1=64, l2=3
        let mut rng = rand::thread_rng();

        // Generate 67 random seeds
        let mut private_key = Vec::with_capacity(67);
        for _ in 0..67 {
            let mut seed = [0u8; 32];
            rng.fill(&mut seed);
            private_key.push(seed);
        }

        // Derive public key: hash each seed 15 times (w-1)
        let public_key: Vec<[u8; 32]> = private_key
            .iter()
            .map(|seed| chain_hash(seed, params.w - 1))
            .collect();

        Self { private_key, public_key, used: false, params }
    }

    pub fn sign(&mut self, digest: &[u8; 32]) -> Result<WotsSignature, WotsError> {
        // CRITICAL: Enforce one-time property
        if self.used {
            return Err(WotsError::KeyAlreadyUsed);
        }
        self.used = true;

        // Convert digest to base-16 digits
        let msg_indices = self.base_w_encode(digest);

        // Compute checksum (prevents forgery)
        let checksum = self.compute_checksum(&msg_indices);
        let checksum_indices = self.encode_checksum(checksum);

        // Generate signature: reveal intermediate chain values
        let signature_chains: Vec<[u8; 32]> = self.private_key
            .iter()
            .zip(msg_indices.iter().chain(checksum_indices.iter()))
            .map(|(seed, &index)| chain_hash(seed, index))
            .collect();

        Ok(WotsSignature::new(signature_chains, self.params.w, digest.to_vec()))
    }
}
```

### Signature Verification: src/wots/signature.rs

```rust
impl WotsSignature {
    pub fn verify(&self, public_key: &[u8], digest: &[u8; 32]) -> Result<(), WotsError> {
        // Check digest matches what was signed
        if self.signed_digest != digest {
            return Err(WotsError::VerificationFailed);
        }

        // Encode message same way as signing
        let msg_indices = self.base_w_encode(digest, &params);
        let checksum = self.compute_checksum(&msg_indices, &params);
        let checksum_indices = self.encode_checksum(checksum, &params);

        // Verify: hash signature forward, should reach public key
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
}
```

---

## How to Make Changes {#how-to-make-changes}

### Step 1: Make Your Changes

Edit the relevant `.rs` files.

### Step 2: Check for Compile Errors

```bash
cargo check
```

This is faster than `cargo build` - it only checks syntax and types.

### Step 3: Build

```bash
cargo build
```

### Step 4: Run Tests

```bash
cargo test
```

### Step 5: Run the Application

```bash
cargo run
```

### Step 6: Format Code (Optional but Recommended)

```bash
cargo fmt
```

This automatically formats code to Rust standards.

### Step 7: Check for Common Issues

```bash
cargo clippy
```

This checks for common mistakes and suggests improvements.

---

## Adding New Features {#adding-new-features}

### Example: Add a New Hash Function

#### Step 1: Add Dependency

Edit `Cargo.toml`:
```toml
[dependencies]
# ... existing deps ...
sha2 = "0.10"  # Add SHA-256
```

#### Step 2: Create the Code

Edit `src/wots/mod.rs`:
```rust
// Add new hash function
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
```

#### Step 3: Add Tests

Edit `src/wots/mod.rs` (in the tests module):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let result = hash_sha256(b"test");
        assert_eq!(result.len(), 32);
    }
}
```

#### Step 4: Run Tests

```bash
cargo test test_sha256_hash
```

### Example: Add a New Command-Line Argument

#### Step 1: Add Dependency

Edit `Cargo.toml`:
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

#### Step 2: Modify main.rs

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Run only key generation demo
    #[arg(long)]
    keygen_only: bool,
}

fn main() {
    let args = Args::parse();

    if args.keygen_only {
        demo_key_generation();
    } else {
        // Run full demo
        // ...
    }
}
```

#### Step 3: Test

```bash
cargo run -- --keygen-only
cargo run -- --help
```

---

## Testing Guide {#testing-guide}

### Types of Tests

| Type | Location | Purpose |
|------|----------|---------|
| Unit tests | Same file as code | Test individual functions |
| Integration tests | `tests/` directory | Test public API |

### Writing Unit Tests

```rust
// In src/wots/keypair.rs

#[cfg(test)]  // Only compiled during tests
mod tests {
    use super::*;  // Import from parent module

    #[test]  // Mark as test
    fn test_keypair_generation() {
        let keypair = WotsKeypair::generate();
        assert!(!keypair.is_used());
        assert_eq!(keypair.public_key_size(), 2144);
    }

    #[test]
    fn test_signing_marks_key_used() {
        let mut keypair = WotsKeypair::generate();
        let digest = [0u8; 32];
        keypair.sign(&digest).unwrap();
        assert!(keypair.is_used());
    }

    #[test]
    #[should_panic]  // Test that expects panic
    fn test_invalid_input_panics() {
        // ...
    }
}
```

### Writing Integration Tests

```rust
// In tests/wots_tests.rs

use post_quantum_cryptography::wots::{WotsKeypair, WotsError};

#[test]
fn test_one_time_property() {
    let mut keypair = WotsKeypair::generate();

    let digest1 = blake3::hash(b"First").into();
    let digest2 = blake3::hash(b"Second").into();

    // First signature succeeds
    assert!(keypair.sign(&digest1).is_ok());

    // Second signature fails
    assert!(matches!(
        keypair.sign(&digest2),
        Err(WotsError::KeyAlreadyUsed)
    ));
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_one_time_property

# Tests in specific file
cargo test --test wots_tests

# With output shown
cargo test -- --nocapture

# Only tests matching pattern
cargo test keypair
```

---

## Debugging {#debugging}

### Print Debugging

```rust
// Basic print
println!("Value: {:?}", some_value);

// Pretty print (formatted)
println!("Value: {:#?}", some_struct);

// Debug macro (includes file/line)
dbg!(some_value);
```

### Using VS Code Debugger

1. Install **CodeLLDB** extension
2. Create `.vscode/launch.json`:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": ["build", "--bin=pqc-demo"]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug Tests",
            "cargo": {
                "args": ["test", "--no-run"]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```
3. Set breakpoints by clicking left of line numbers
4. Press F5 to start debugging

### Common Debugging Scenarios

**"Why is my value wrong?"**
```rust
dbg!(&my_variable);  // Prints value and location
```

**"Is this code path reached?"**
```rust
println!(">>> Reached point A");
```

**"What's in this vector?"**
```rust
for (i, item) in my_vec.iter().enumerate() {
    println!("Item {}: {:?}", i, item);
}
```

---

## Common Patterns in This Codebase {#common-patterns}

### Pattern 1: Builder-Style Methods

```rust
let tx = Transaction::new(from, to, 1000, 0)
    .with_data(data);  // Returns Self for chaining
```

### Pattern 2: Error Handling with ?

```rust
pub fn process(&mut self) -> Result<(), BlockchainError> {
    self.sign(&mut keypair)?;   // Returns early if error
    self.verify_signature()?;   // Returns early if error
    Ok(())
}
```

### Pattern 3: Module Re-exports

```rust
// In mod.rs
mod keypair;
pub use keypair::WotsKeypair;  // Re-export for cleaner imports
```

### Pattern 4: Configuration with Default

```rust
impl Default for WotsParams {
    fn default() -> Self {
        Self { w: 16, n: 32, l1: 64, l2: 3 }
    }
}

// Usage:
let params = WotsParams::default();
```

### Pattern 5: Private Constructors

```rust
impl WotsSignature {
    // Only callable from within the crate
    pub(crate) fn new(...) -> Self { ... }
}
```

---

## Extending the Project {#extending-the-project}

### Idea 1: Add SPHINCS+ Support

SPHINCS+ is a stateless hash-based signature (uses WOTS+ internally).

```rust
// Create new module: src/sphincs/mod.rs
pub struct SphincsKeypair { ... }
pub struct SphincsSignature { ... }
```

### Idea 2: Add STARK Proof Generation

Use the `winterfell` crate to generate proofs.

```rust
// In Cargo.toml
winterfell = "0.10"

// Create src/stark/mod.rs
// Implement AIR trait for hash chain verification
```

### Idea 3: Add Command-Line Interface

Use `clap` for argument parsing.

```rust
// Options: benchmark mode, different hash functions, etc.
cargo run -- --benchmark --iterations 1000
```

### Idea 4: Add Web Interface

Use `wasm-bindgen` to compile to WebAssembly.

```rust
// In Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

### Idea 5: Add Benchmarks

```rust
// In benches/benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_keygen(c: &mut Criterion) {
    c.bench_function("keygen", |b| {
        b.iter(|| WotsKeypair::generate())
    });
}

criterion_group!(benches, benchmark_keygen);
criterion_main!(benches);
```

---

## Sample Outputs

Pre-captured outputs are available in `examples/`:
- **[Demo Output](examples/demo_results.txt)** - Complete `cargo run` output
- **[Test Results](examples/test_results.txt)** - Complete `cargo test` output (48 tests)

---

## Getting Help

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/) - Official tutorial
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### Cryptography Resources

- [WOTS+ RFC 8391](https://datatracker.ietf.org/doc/html/rfc8391)
- [Understanding WOTS+](https://hackmd.io/@0xdeveloperuche/Bk8UzubUel)
- [Tutorials](../tutorials/00-INDEX.md) - 11 tutorials from beginner to PhD level

### Project-Specific Help

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for design decisions
- Read [CLAUDE.md](../CLAUDE.md) for AI assistant guidance
- Check existing tests for usage examples
