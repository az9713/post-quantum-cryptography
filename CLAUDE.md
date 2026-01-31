# CLAUDE.md - Instructions for Claude Code

This file provides guidance for AI assistants (Claude, GPT, etc.) working with this codebase.

## Project Overview

**Name:** Post-Quantum Cryptography Demo
**Language:** Rust
**Purpose:** Educational demonstration of WOTS+ post-quantum signatures
**Status:** Complete, functional

**Scope Clarification:**
- `wots/` module: **Production-quality** WOTS+ cryptography (RFC 8391)
- `blockchain/` module: **Minimal simulation** (transaction objects only, no actual blockchain)
- `demo/` module: Educational output explaining quantum threats

## Quick Commands

```bash
# Build
cargo build

# Run tests (all 48)
cargo test

# Run demo
cargo run

# Check for errors without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Structure

```
post-quantum-cryptography/
├── Cargo.toml           # Dependencies and project config
├── src/
│   ├── lib.rs           # Library exports
│   ├── main.rs          # Demo entry point
│   ├── wots/            # WOTS+ cryptography
│   │   ├── mod.rs       # Module root, WotsParams, WotsError, hash functions
│   │   ├── keypair.rs   # WotsKeypair: generate(), sign(), is_used()
│   │   └── signature.rs # WotsSignature: verify(), to_bytes(), from_bytes()
│   ├── blockchain/      # Transaction simulation
│   │   ├── mod.rs       # BlockchainError
│   │   ├── address.rs   # Address: from_keypair(), to_string()
│   │   └── transaction.rs # Transaction: sign(), verify_signature()
│   └── demo/            # Educational output
│       ├── mod.rs
│       ├── educational.rs # Colored explanations
│       └── timing.rs    # Performance measurement
├── tests/               # Integration tests
│   ├── wots_tests.rs    # 9 WOTS+ tests
│   └── blockchain_tests.rs # 14 blockchain tests
├── docs/                # Documentation
│   ├── examples/        # Captured outputs
│   │   ├── demo_results.txt   # cargo run output
│   │   └── test_results.txt   # cargo test output
│   └── *.md             # Guides
└── tutorials/           # 11 tutorials (00-INDEX through 10-PHD)
```

## Key Types

### WotsKeypair
- `generate()` - Create new keypair (67 hash chains)
- `sign(&mut self, digest: &[u8; 32])` - Sign (marks key as used)
- `is_used()` - Check if already signed
- `public_key_bytes()` - Get 2144-byte public key

### WotsSignature
- `verify(&self, public_key: &[u8], digest: &[u8; 32])` - Verify signature
- `to_bytes()` / `from_bytes()` - Serialization
- `size_bytes()` - Returns 2144

### Transaction
- `new(from, to, amount, nonce)` - Create unsigned transaction
- `sign(&mut self, keypair)` - Sign with WOTS+ (keypair must match `from`)
- `verify_signature()` - Verify the signature
- `compute_digest()` - SHA3-256 hash of fields

### Address
- `from_keypair(keypair)` - Derive address from public key
- `to_string()` - "qw1" + hex encoding
- `from_string(s)` - Parse address

## Critical Security Properties

1. **One-Time Signatures:** `WotsKeypair::sign()` sets `self.used = true` and subsequent calls return `Err(WotsError::KeyAlreadyUsed)`. Never bypass this.

2. **Checksum:** The checksum in `base_w_encode` prevents forgery. Don't modify the checksum calculation.

3. **Key Binding:** `Transaction::sign()` verifies the keypair matches the `from` address. Don't skip this check.

## Common Tasks

### Add a New Test

```rust
// In tests/wots_tests.rs or tests/blockchain_tests.rs

#[test]
fn test_your_new_test() {
    // Setup
    let mut keypair = WotsKeypair::generate();

    // Action
    let digest = blake3::hash(b"test").into();
    let result = keypair.sign(&digest);

    // Assertion
    assert!(result.is_ok());
}
```

Run with: `cargo test test_your_new_test`

### Modify Key Generation

Edit `src/wots/keypair.rs`:
```rust
impl WotsKeypair {
    pub fn generate() -> Self {
        // Modify here
    }
}
```

### Add New Hash Function

Edit `src/wots/mod.rs`:
```rust
pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    Sha256::new().chain_update(data).finalize().into()
}
```

### Add New Demo Section

Edit `src/demo/educational.rs`:
```rust
pub fn explain_new_concept() {
    print_education_box(
        "YOUR TITLE",
        "Your explanation here..."
    );
}
```

Then call from `src/main.rs`.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| blake3 | 1.5 | Primary hash function |
| sha3 | 0.10 | Transaction digest |
| rand | 0.8 | Key generation |
| serde | 1.0 | Serialization |
| serde_json | 1.0 | JSON support |
| hex | 0.4 | Hex encoding |
| colored | 2.0 | Terminal colors |
| thiserror | 1.0 | Error types |
| anyhow | 1.0 | Error handling |

## Coding Conventions

1. **Error Handling:** Use `Result<T, E>` with custom error types
2. **Documentation:** Add `///` doc comments to public items
3. **Testing:** Add tests for new functionality
4. **Formatting:** Run `cargo fmt` before committing

## What NOT to Do

1. **Don't bypass one-time check:** The `used` flag is critical security
2. **Don't use weak RNG:** Use `rand::thread_rng()` for key generation
3. **Don't modify checksum logic:** It prevents signature forgery
4. **Don't expose private keys:** They should never leave `WotsKeypair`

## Extending the Project

### Add STARK Proof Generation

```toml
# Cargo.toml
[dependencies]
winterfell = "0.10"

[features]
stark = []
```

### Add CLI Arguments

```toml
# Cargo.toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

### Add Web Interface

```toml
# Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

## Test Coverage

- **WOTS+ (9 tests):** Key generation, signing, verification, one-time property
- **Blockchain (14 tests):** Addresses, transactions, tamper detection
- **Unit (24 tests):** Module-level functionality
- **Demo (1 test):** Integration smoke test

Total: 48 tests, all passing

## Performance Expectations

| Operation | Expected Time |
|-----------|---------------|
| Key generation | 1-5 ms |
| Signing | 100-500 μs |
| Verification | 100-500 μs |

## Troubleshooting

**Build fails:**
```bash
cargo clean
cargo build
```

**Tests fail:**
```bash
cargo test -- --nocapture  # See output
```

**Dependencies outdated:**
```bash
cargo update
```

## Documentation Files

- `README.md` - Project overview
- `docs/QUICK_START.md` - 10 hands-on examples
- `docs/USER_GUIDE.md` - Complete user documentation
- `docs/DEVELOPER_GUIDE.md` - Developer documentation
- `docs/ARCHITECTURE.md` - Technical design
- `docs/examples/demo_results.txt` - Captured demo output
- `docs/examples/test_results.txt` - Captured test results
- `tutorials/00-INDEX.md` - Tutorial index (11 tutorials, beginner to PhD)

## Contact / Context

This project demonstrates concepts from:
- a16z crypto research on quantum threats to blockchain
- Ethereum Foundation post-quantum initiatives (January 2026)
- NIST post-quantum cryptography standards (FIPS 203/204/205)

The code is educational, not production-ready.
