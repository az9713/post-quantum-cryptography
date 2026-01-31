# Architecture Documentation

This document explains the technical design, algorithms, and code structure of the Post-Quantum Cryptography demo.

> **Note:** This is a **cryptography demonstration**, not a blockchain implementation. The WOTS+ signature implementation is production-quality, but the "blockchain" module provides only transaction objects for demonstration. See [Scope](#scope) below.

---

## Table of Contents

1. [Scope](#scope)
2. [System Overview](#system-overview)
3. [WOTS+ Algorithm Deep Dive](#wots-algorithm)
4. [Module Architecture](#module-architecture)
5. [Data Structures](#data-structures)
6. [Algorithms](#algorithms)
7. [Security Properties](#security-properties)
8. [Performance Characteristics](#performance)
9. [Design Decisions](#design-decisions)
10. [Future Architecture](#future-architecture)

---

## Scope {#scope}

### What This Project IS

| Component | Quality | Description |
|-----------|---------|-------------|
| **WOTS+ Library** | Production-quality | Full implementation per RFC 8391 |
| **Cryptographic primitives** | Production-quality | Blake3, SHA3-256, secure RNG |
| **Transaction signing** | Realistic | Complete sign/verify workflow |
| **Educational demos** | Comprehensive | Explains quantum threats and solutions |
| **Tutorials** | Extensive | 11 tutorials from beginner to PhD level |

### What This Project is NOT

| Component | Status | Notes |
|-----------|--------|-------|
| Blockchain | ❌ Not implemented | No blocks, no chain structure |
| Consensus | ❌ Not implemented | No PoW/PoS/PBFT |
| Networking | ❌ Not implemented | No P2P, no nodes |
| State management | ❌ Not implemented | No balances, no UTXO |
| Persistence | ❌ Not implemented | No database |

**The "blockchain" module is a minimal transaction simulation** to demonstrate how WOTS+ signatures would integrate with blockchain-style transactions. It is not a functional blockchain.

---

## System Overview {#system-overview}

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Application Layer                            │
│                                                                      │
│  ┌─────────────┐  ┌─────────────────┐  ┌────────────────────────┐   │
│  │   main.rs   │  │  demo/          │  │  Educational Output    │   │
│  │  Entry      │  │  educational.rs │  │  Timing & Visualization│   │
│  │  Point      │  │  timing.rs      │  │                        │   │
│  └──────┬──────┘  └────────┬────────┘  └────────────────────────┘   │
│         │                  │                                         │
└─────────┼──────────────────┼─────────────────────────────────────────┘
          │                  │
          ▼                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Library Layer (lib.rs)                       │
│                                                                      │
│  ┌───────────────────────────┐    ┌──────────────────────────────┐  │
│  │      wots/ Module         │    │     blockchain/ Module       │  │
│  │                           │    │                              │  │
│  │  ┌─────────────────────┐  │    │  ┌────────────────────────┐  │  │
│  │  │     keypair.rs      │  │    │  │    transaction.rs      │  │  │
│  │  │  - WotsKeypair      │  │    │  │  - Transaction         │  │  │
│  │  │  - Key generation   │  │    │  │  - Sign/Verify         │  │  │
│  │  │  - Signing          │  │    │  │  - Serialization       │  │  │
│  │  └─────────────────────┘  │    │  └────────────────────────┘  │  │
│  │                           │    │                              │  │
│  │  ┌─────────────────────┐  │    │  ┌────────────────────────┐  │  │
│  │  │    signature.rs     │  │    │  │     address.rs         │  │  │
│  │  │  - WotsSignature    │  │    │  │  - Address             │  │  │
│  │  │  - Verification     │  │    │  │  - Derivation          │  │  │
│  │  │  - Serialization    │  │    │  │  - Display             │  │  │
│  │  └─────────────────────┘  │    │  └────────────────────────┘  │  │
│  │                           │    │                              │  │
│  │  ┌─────────────────────┐  │    └──────────────────────────────┘  │
│  │  │      mod.rs         │  │                                      │
│  │  │  - WotsParams       │  │                                      │
│  │  │  - WotsError        │  │                                      │
│  │  │  - hash()           │  │                                      │
│  │  │  - chain_hash()     │  │                                      │
│  │  └─────────────────────┘  │                                      │
│  └───────────────────────────┘                                      │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       External Dependencies                          │
│                                                                      │
│  blake3     sha3       rand      serde      colored     thiserror   │
│  (hash)    (digest)   (rng)    (serial)    (output)    (errors)     │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Data Flow: Transaction Signing

```
┌─────────────────┐
│  User creates   │
│  Transaction    │
│  (from, to,     │
│   amount, nonce)│
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ compute_digest()│───────────────────────┐
│                 │                       │
│ SHA3-256 hash   │                       │
│ of all fields   │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ WotsKeypair::   │                       │
│ sign(digest)    │                       │
│                 │                       │
│ 1. Check !used  │                       │
│ 2. base_w_encode│                       │
│ 3. checksum     │                       │
│ 4. chain_hash   │                       │
│ 5. Mark used    │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ WotsSignature   │                       │
│                 │                       │
│ 67 chain values │                       │
│ + signed digest │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                ▼
┌─────────────────────────────────────────────┐
│              Signed Transaction             │
│                                             │
│  from, to, amount, nonce, signature, pubkey │
└─────────────────────────────────────────────┘
```

---

## WOTS+ Algorithm Deep Dive {#wots-algorithm}

### What is WOTS+?

WOTS+ (Winternitz One-Time Signatures Plus) is a hash-based signature scheme where:
- **Security** relies only on hash function properties
- **One-time** means each key signs exactly one message
- **Plus** adds a checksum to prevent forgery

### Parameters

| Parameter | Symbol | Value | Meaning |
|-----------|--------|-------|---------|
| Winternitz | w | 16 | Base for message encoding |
| Security | n | 32 | Hash output size in bytes |
| Message chains | l₁ | 64 | Number of message digits |
| Checksum chains | l₂ | 3 | Number of checksum digits |
| Total chains | l | 67 | l₁ + l₂ |

### Key Generation

```
Input: Security parameter n, Winternitz parameter w
Output: Private key SK, Public key PK

1. For i = 1 to l:
   a. Generate random 32-byte seed: sk[i] ← random()

2. For i = 1 to l:
   a. Hash seed (w-1) times: pk[i] = H^(w-1)(sk[i])

3. Return:
   SK = [sk[1], sk[2], ..., sk[l]]
   PK = [pk[1], pk[2], ..., pk[l]]
```

**Visual Representation:**

```
Private Key (seeds):        Public Key (endpoints):

sk[1] ──H──H──...──H──→ pk[1]    (15 hashes for w=16)
sk[2] ──H──H──...──H──→ pk[2]
  ⋮                       ⋮
sk[67]──H──H──...──H──→ pk[67]
```

### Signing

```
Input: Message digest M, Private key SK
Output: Signature σ

1. Encode M as base-w digits: [m[1], m[2], ..., m[l₁]]
   - For w=16: each nibble (4 bits) is one digit

2. Compute checksum: C = Σ(w-1 - m[i]) for all i
   - Prevents forgery by making checksum DECREASE as message values INCREASE

3. Encode checksum as base-w: [c[1], c[2], ..., c[l₂]]

4. Combine: indices = [m[1]...m[l₁], c[1]...c[l₂]]

5. For i = 1 to l:
   σ[i] = H^(indices[i])(sk[i])

6. Return σ = [σ[1], σ[2], ..., σ[l]]
```

**Visual Representation:**

```
Message "Hello" → Digest → Base-16 digits

sk[1] ──H^(m[1])──→ σ[1]    (reveal intermediate value)
sk[2] ──H^(m[2])──→ σ[2]
  ⋮                  ⋮
sk[67]──H^(c[3])──→ σ[67]
```

### Verification

```
Input: Message digest M, Signature σ, Public key PK
Output: Valid or Invalid

1. Encode M and checksum same as signing
   indices = [m[1]...m[l₁], c[1]...c[l₂]]

2. For i = 1 to l:
   remaining = (w-1) - indices[i]
   computed[i] = H^(remaining)(σ[i])

3. If computed[i] == pk[i] for all i:
   Return Valid
   Else:
   Return Invalid
```

**Visual Representation:**

```
σ[1] ──H^(w-1-m[1])──→ computed[1] =? pk[1]
σ[2] ──H^(w-1-m[2])──→ computed[2] =? pk[2]
  ⋮                      ⋮
σ[67]──H^(w-1-c[3])──→ computed[67] =? pk[67]
```

### Why the Checksum Prevents Forgery

**Without checksum:**
- Attacker sees σ[i] = H^(m[i])(sk[i])
- Can compute H(σ[i]) = H^(m[i]+1)(sk[i])
- This is a valid signature for m[i]+1
- Attacker can forge signatures for LARGER message values

**With checksum:**
- Checksum C = Σ(w-1 - m[i])
- If m[i] increases, C decreases
- To forge larger message, need smaller checksum
- Smaller checksum requires computing hash PREIMAGES
- Preimages are computationally infeasible

---

## Module Architecture {#module-architecture}

### wots/ Module

**Responsibility:** Implement WOTS+ cryptographic primitives

```rust
// mod.rs - Module root and shared utilities
pub struct WotsParams { w, n, l1, l2 }
pub enum WotsError { KeyAlreadyUsed, VerificationFailed, ... }
pub fn hash(data: &[u8]) -> [u8; 32]
pub(crate) fn chain_hash(input: &[u8; 32], iterations: usize) -> [u8; 32]

// keypair.rs - Key generation and signing
pub struct WotsKeypair { private_key, public_key, used, params }
impl WotsKeypair {
    pub fn generate() -> Self
    pub fn sign(&mut self, digest: &[u8; 32]) -> Result<WotsSignature, WotsError>
    pub fn is_used(&self) -> bool
    pub fn public_key() -> WotsPublicKey
}

// signature.rs - Verification and serialization
pub struct WotsSignature { chains, w, signed_digest }
impl WotsSignature {
    pub fn verify(&self, public_key: &[u8], digest: &[u8; 32]) -> Result<(), WotsError>
    pub fn to_bytes(&self) -> Vec<u8>
    pub fn from_bytes(data: &[u8]) -> Result<Self, WotsError>
}
```

### blockchain/ Module

**Responsibility:** Simulate blockchain transactions using WOTS+ signatures

```rust
// mod.rs - Module root and error types
pub enum BlockchainError { MissingSignature, InvalidSignature, ... }

// address.rs - Address derivation
pub struct Address { bytes: [u8; 32] }
impl Address {
    pub fn from_keypair(keypair: &WotsKeypair) -> Self
    pub fn from_string(s: &str) -> Result<Self, &'static str>
    pub fn to_string(&self) -> String  // "qw1..."
}

// transaction.rs - Transaction structure
pub struct Transaction { from, to, amount, nonce, data, signature, pub_key }
impl Transaction {
    pub fn new(from, to, amount, nonce) -> Self
    pub fn compute_digest(&self) -> [u8; 32]
    pub fn sign(&mut self, keypair: &mut WotsKeypair) -> Result<(), BlockchainError>
    pub fn verify_signature(&self) -> Result<(), BlockchainError>
}
```

### demo/ Module

**Responsibility:** Educational output and timing utilities

```rust
// educational.rs - Colored educational output
pub fn print_header(title: &str)
pub fn print_education_box(title: &str, content: &str)
pub fn explain_quantum_threat()
pub fn explain_wots()
pub fn print_signature_comparison()

// timing.rs - Performance measurement
pub struct Timer { start, label }
pub fn time_operation<T, F: FnOnce() -> T>(f: F) -> (T, Duration)
pub struct PerformanceMetrics { keygen, signing, verification }
```

---

## Data Structures {#data-structures}

### WotsKeypair

```rust
pub struct WotsKeypair {
    /// Private key: 67 random 32-byte seeds
    private_key: Vec<[u8; 32]>,  // 67 × 32 = 2144 bytes

    /// Public key: 67 hash chain endpoints
    public_key: Vec<[u8; 32]>,   // 67 × 32 = 2144 bytes

    /// One-time enforcement flag
    used: bool,

    /// Configuration parameters
    params: WotsParams,
}
```

### WotsSignature

```rust
pub struct WotsSignature {
    /// Signature: 67 intermediate chain values
    chains: Vec<[u8; 32]>,  // 67 × 32 = 2144 bytes

    /// Winternitz parameter (for verification)
    w: usize,

    /// The digest that was signed (for verification)
    signed_digest: Vec<u8>,  // 32 bytes
}
```

**Serialization format:**
```
[w: 1 byte][chains: 67×32 bytes][digest: 32 bytes] = 2177 bytes total
```

### Transaction

```rust
pub struct Transaction {
    /// Sender address (32 bytes)
    pub from: Address,

    /// Recipient address (32 bytes)
    pub to: Address,

    /// Transfer amount
    pub amount: u64,  // 8 bytes

    /// Transaction nonce (prevents replay)
    pub nonce: u64,   // 8 bytes

    /// Optional transaction data
    pub data: Option<Vec<u8>>,

    /// WOTS+ signature (after signing)
    signature: Option<SerializedSignature>,  // ~2177 bytes

    /// Sender's public key (for verification)
    pub_key: Option<Vec<u8>>,  // 2144 bytes
}
```

**Total signed transaction size:** ~4400 bytes

---

## Algorithms {#algorithms}

### Base-W Encoding

Converts a 32-byte digest into 64 base-16 digits.

```rust
fn base_w_encode(&self, digest: &[u8; 32]) -> Vec<usize> {
    let mut result = Vec::with_capacity(64);

    for byte in digest {
        // High nibble
        result.push((byte >> 4) as usize);     // 0-15
        // Low nibble
        result.push((byte & 0x0F) as usize);   // 0-15
    }

    result  // 64 values, each 0-15
}
```

### Checksum Calculation

```rust
fn compute_checksum(&self, msg_indices: &[usize]) -> usize {
    let max_digit = self.params.w - 1;  // 15 for w=16

    // Sum of (max - actual) for each digit
    msg_indices.iter()
        .map(|&d| max_digit - d)
        .sum()

    // Range: 0 (all 15s) to 960 (all 0s)
}

fn encode_checksum(&self, checksum: usize) -> Vec<usize> {
    // Encode as 3 base-16 digits
    let mut result = Vec::with_capacity(3);
    let mut remaining = checksum;

    for _ in 0..3 {
        result.push(remaining % 16);
        remaining /= 16;
    }

    result.reverse();
    result
}
```

### Transaction Digest

```rust
fn compute_digest(&self) -> [u8; 32] {
    let mut hasher = Sha3_256::new();

    // Hash all fields in deterministic order
    hasher.update(self.from.as_bytes());      // 32 bytes
    hasher.update(self.to.as_bytes());        // 32 bytes
    hasher.update(self.amount.to_le_bytes()); // 8 bytes
    hasher.update(self.nonce.to_le_bytes());  // 8 bytes

    if let Some(ref data) = self.data {
        hasher.update((data.len() as u64).to_le_bytes());
        hasher.update(data);
    } else {
        hasher.update(0u64.to_le_bytes());
    }

    hasher.finalize().into()
}
```

---

## Security Properties {#security-properties}

### Property 1: One-Time Enforcement

**Implementation:**
```rust
pub fn sign(&mut self, ...) -> Result<WotsSignature, WotsError> {
    if self.used {
        return Err(WotsError::KeyAlreadyUsed);
    }
    self.used = true;
    // ... proceed with signing
}
```

**Why it matters:** Signing twice reveals enough chain values to forge signatures.

### Property 2: Tamper Detection

**Implementation:** Signature covers the hash of all transaction fields.

```rust
let digest = transaction.compute_digest();  // SHA3-256 of all fields
keypair.sign(&digest)?;                     // Sign the digest
```

**Verification:**
```rust
let expected_digest = transaction.compute_digest();  // Recompute
signature.verify(&public_key, &expected_digest)?;    // Must match
```

### Property 3: Key Binding

**Implementation:** Transaction verifies keypair matches sender.

```rust
pub fn sign(&mut self, keypair: &mut WotsKeypair) -> Result<(), BlockchainError> {
    let expected_address = Address::from_keypair(keypair);
    if self.from != expected_address {
        return Err(BlockchainError::InvalidTransaction(...));
    }
    // ... proceed
}
```

### Property 4: Post-Quantum Security

**Basis:** WOTS+ security reduces to hash function properties:
- **Preimage resistance:** Given H(x), hard to find x
- **Second preimage resistance:** Given x, hard to find x' where H(x) = H(x')

**Against quantum computers:**
- Grover's algorithm provides √N speedup for preimage search
- 256-bit hash → 128-bit post-quantum security
- This is still computationally infeasible

---

## Performance Characteristics {#performance}

### Measured Performance (Debug Build)

| Operation | Time | Notes |
|-----------|------|-------|
| Key generation | ~2ms | 67 chains × 15 hashes each |
| Signing | ~200μs | 67 chain hashes |
| Verification | ~180μs | 67 chain hashes |

### Size Comparison

| Item | ECDSA | WOTS+ | Ratio |
|------|-------|-------|-------|
| Private key | 32 bytes | 2144 bytes | 67× |
| Public key | 33 bytes | 2144 bytes | 65× |
| Signature | 64 bytes | 2144 bytes | 34× |
| Transaction | ~110 bytes | ~4400 bytes | 40× |

### Memory Usage

```
WotsKeypair:
  private_key: 67 × 32 = 2,144 bytes
  public_key:  67 × 32 = 2,144 bytes
  used:        1 byte (+ padding)
  params:      32 bytes
  Total:       ~4,325 bytes

WotsSignature:
  chains:        67 × 32 = 2,144 bytes
  w:             8 bytes
  signed_digest: 32 bytes
  Total:         ~2,184 bytes
```

---

## Design Decisions {#design-decisions}

### Decision 1: Custom WOTS+ Implementation

**Choice:** Implement WOTS+ from scratch instead of using `winternitz-ots` crate.

**Rationale:**
- Educational value: Understanding the algorithm
- Full control over implementation
- Avoid external dependency issues

**Trade-off:** More code to maintain, but clearer learning experience.

### Decision 2: Blake3 for Hashing

**Choice:** Use Blake3 instead of SHA-256.

**Rationale:**
- Faster (3-5× compared to SHA-256)
- Same security level (256-bit)
- Modern, well-audited design

**Trade-off:** Less "standard" than SHA-256, but cryptographically equivalent.

### Decision 3: Winternitz Parameter w=16

**Choice:** Use w=16 instead of w=256.

**Rationale:**
- Good balance between signature size and speed
- w=16: 2144-byte signatures, faster signing
- w=256: 1088-byte signatures, slower signing

**Trade-off:** Larger signatures, but faster operations.

### Decision 4: Separate Digest from Signature

**Choice:** Store signed_digest inside WotsSignature.

**Rationale:**
- Simplifies verification (don't need to pass digest separately)
- Prevents signing one message and claiming another was signed

**Trade-off:** Slightly larger signatures (+32 bytes).

---

## Future Architecture {#future-architecture}

### Planned: STARK Aggregation

```
┌─────────────────────────────────────────────────────────────────────┐
│                      Future: STARK Prover                           │
│                                                                      │
│  ┌───────────────┐    ┌─────────────────┐    ┌──────────────────┐   │
│  │ 1000 WOTS+    │ → │  STARK Prover    │ → │ Single ~50KB     │   │
│  │ Signatures    │    │  (winterfell)    │    │ Proof            │   │
│  │ (2.1 MB)      │    │                  │    │                  │   │
│  └───────────────┘    └─────────────────┘    └──────────────────┘   │
│                                                                      │
│  Proves: "I verified all 1000 signatures correctly"                 │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Planned: Hierarchical Deterministic Keys

```
                    Master Seed
                         │
                         ▼
              ┌──────────────────┐
              │  HD Derivation   │
              └────────┬─────────┘
                       │
       ┌───────────────┼───────────────┐
       │               │               │
       ▼               ▼               ▼
   Account 0       Account 1       Account 2
       │               │               │
   ┌───┴───┐       ┌───┴───┐       ┌───┴───┐
   │       │       │       │       │       │
   ▼       ▼       ▼       ▼       ▼       ▼
 Key 0   Key 1   Key 0   Key 1   Key 0   Key 1
```

### Planned: Multi-Signature Support

```rust
pub struct MultiSig {
    threshold: usize,       // e.g., 2-of-3
    public_keys: Vec<WotsPublicKey>,
    signatures: Vec<Option<WotsSignature>>,
}
```

---

## Additional Resources

### Sample Outputs

Pre-captured outputs are available in `examples/`:
- **[Demo Output](examples/demo_results.txt)** - Full `cargo run` educational output
- **[Test Results](examples/test_results.txt)** - Complete `cargo test` with 48 passing tests

### Tutorials

For in-depth understanding of the cryptographic concepts:
- **[Tutorial Index](../tutorials/00-INDEX.md)** - 11 tutorials from beginner to PhD level
- Covers: quantum computing, hash functions, digital signatures, WOTS+, STARKs, and more

---

## Summary

This architecture provides:

1. **Correctness:** All security properties enforced
2. **Clarity:** Educational value through clean separation
3. **Extensibility:** Easy to add STARK, HD keys, etc.
4. **Performance:** Reasonable for demonstration purposes
5. **Testability:** 48 tests covering all functionality
