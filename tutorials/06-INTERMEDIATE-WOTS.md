# Intermediate Tutorial: WOTS+ (Winternitz One-Time Signatures)

**Time:** 60-90 minutes
**Prerequisites:** Tutorial 04 (Hash Functions), basic understanding of signatures
**Goal:** Fully understand how WOTS+ works and why it's quantum-resistant

---

## Table of Contents

1. [The Idea: Signatures from Hash Functions](#the-idea)
2. [Lamport Signatures](#lamport)
3. [Winternitz Optimization](#winternitz)
4. [WOTS+ (The Plus Variant)](#wots-plus)
5. [The Checksum: Preventing Forgery](#checksum)
6. [Complete WOTS+ Algorithm](#complete-algorithm)
7. [Security Analysis](#security)
8. [Implementation in This Project](#implementation)
9. [Comparison with Other Schemes](#comparison)
10. [Exercises](#exercises)

---

## The Idea: Signatures from Hash Functions {#the-idea}

### Why Hash-Based Signatures?

```
Traditional signatures (RSA, ECDSA):
  Security based on: Mathematical problems (factoring, discrete log)
  Quantum status:    BROKEN by Shor's algorithm

Hash-based signatures:
  Security based on: Hash function properties (preimage resistance)
  Quantum status:    SECURE (Grover only gives √ speedup)
```

### The Core Insight

If you have a secret value `s`, you can:
1. Publish `H(s)` (the hash)
2. Later reveal `s` to prove you knew it

```
Commitment:  h = H(s)    ← Anyone can compute this
Revelation:  s           ← Only you know this initially

Verification: Is H(s) = h?
```

This is the foundation of hash-based signatures.

---

## Lamport Signatures {#lamport}

### The Simplest Hash-Based Signature (1979)

Leslie Lamport invented the first one-time signature scheme.

### Key Generation

For each bit of the message digest (256 bits), generate TWO random secrets:

```
Private key (for 256-bit message):
  SK = [(s₀⁰, s₀¹), (s₁⁰, s₁¹), ..., (s₂₅₅⁰, s₂₅₅¹)]
       ↑
       512 random 256-bit values

Public key (hash each secret):
  PK = [(H(s₀⁰), H(s₀¹)), (H(s₁⁰), H(s₁¹)), ..., (H(s₂₅₅⁰), H(s₂₅₅¹))]
```

### Signing

For each bit i of the message hash, reveal ONE secret:

```
Message hash m = b₀b₁b₂...b₂₅₅ (each bᵢ is 0 or 1)

Signature:
  σ = [s₀^(b₀), s₁^(b₁), ..., s₂₅₅^(b₂₅₅)]

If bit i is 0: reveal sᵢ⁰
If bit i is 1: reveal sᵢ¹
```

### Verification

Check that each revealed secret hashes to the correct public key component:

```
For each bit i of message hash m:
  If bᵢ = 0: Check H(σᵢ) = PKᵢ⁰
  If bᵢ = 1: Check H(σᵢ) = PKᵢ¹
```

### Example (4-bit message)

```
Message hash: 1011

Private key:
  Bit 0: (s₀⁰, s₀¹)
  Bit 1: (s₁⁰, s₁¹)
  Bit 2: (s₂⁰, s₂¹)
  Bit 3: (s₃⁰, s₃¹)

Signature (reveal based on bits):
  Bit 0 = 1: reveal s₀¹
  Bit 1 = 0: reveal s₁⁰
  Bit 2 = 1: reveal s₂¹
  Bit 3 = 1: reveal s₃¹

σ = [s₀¹, s₁⁰, s₂¹, s₃¹]
```

### Lamport: Sizes

For 256-bit security:

| Component | Size |
|-----------|------|
| Private key | 256 × 2 × 32 = 16,384 bytes |
| Public key | 256 × 2 × 32 = 16,384 bytes |
| Signature | 256 × 32 = 8,192 bytes |

**Problem:** These sizes are huge!

---

## Winternitz Optimization {#winternitz}

### The Idea: Use Hash Chains, Not Single Hashes

Instead of one hash per bit, use a hash chain to encode multiple bits at once.

### Hash Chains Review

```
s ──H──▶ H(s) ──H──▶ H²(s) ──H──▶ H³(s) ──H──▶ ... ──H──▶ Hʷ⁻¹(s)
seed                                                       endpoint
```

### The Winternitz Parameter (w)

`w` determines how many bits each chain encodes:
- w = 2: 1 bit per chain (like Lamport)
- w = 4: 2 bits per chain
- w = 16: 4 bits per chain
- w = 256: 8 bits per chain (1 byte)

### How It Works

```
w = 16 means:
  - Each chain encodes 4 bits (values 0-15)
  - Chain length = 16 (indices 0 to 15)
  - Public key = H¹⁵(seed)
  - Signature = Hᵐ(seed) where m ∈ [0, 15]
```

### Key Generation (w = 16, 256-bit hash)

```
Number of chains needed:
  Message bits / bits per chain = 256 / 4 = 64 chains

For each chain i:
  Private key: skᵢ = random 32-byte value
  Public key:  pkᵢ = H¹⁵(skᵢ)   (hash 15 times)
```

### Signing

```
1. Hash message: h = H(message)
2. Split h into base-16 digits: [m₀, m₁, ..., m₆₃]
   Each mᵢ ∈ [0, 15]
3. For each chain i:
   σᵢ = Hᵐⁱ(skᵢ)    (hash mᵢ times)

Signature: σ = [σ₀, σ₁, ..., σ₆₃]
```

### Verification

```
For each chain i:
  Compute: Hʷ⁻¹⁻ᵐⁱ(σᵢ)
  Check:   Result should equal pkᵢ

Why? Because:
  Hʷ⁻¹⁻ᵐⁱ(Hᵐⁱ(skᵢ)) = Hʷ⁻¹(skᵢ) = pkᵢ
```

### Visual Example

```
w = 16, message digit m = 5

Private key:   sk ──H──▶ H(sk) ──▶ ... ──▶ H¹⁵(sk) = pk
                   ↓
Signature:         H⁵(sk) = σ
                            ↓
Verification:              H¹⁰(σ) = H¹⁵(sk) = pk ✓
```

### Size Improvement

For w = 16, 256-bit security:

| Component | Lamport | Winternitz (w=16) | Improvement |
|-----------|---------|-------------------|-------------|
| Private key | 16,384 bytes | 2,048 bytes | 8× smaller |
| Public key | 16,384 bytes | 2,048 bytes | 8× smaller |
| Signature | 8,192 bytes | 2,048 bytes | 4× smaller |

---

## WOTS+ (The Plus Variant) {#wots-plus}

### The Problem with Basic Winternitz

An attacker who sees σᵢ = Hᵐ(sk) can compute:
- Hᵐ⁺¹(sk) = H(σᵢ)
- Hᵐ⁺²(sk) = H(H(σᵢ))
- etc.

This means they can forge signatures for **larger** message digits!

```
Original signature for m = 5: σ = H⁵(sk)
Attacker computes:            σ' = H(σ) = H⁶(sk)

This is a valid signature for m' = 6!
```

### The Checksum Solution

Add extra chains that encode a **checksum** of the message digits.

```
Checksum C = Σ(w - 1 - mᵢ) for all message digits mᵢ

Key property:
  - If any mᵢ INCREASES, C DECREASES
  - To forge larger message, attacker needs smaller checksum
  - Smaller checksum requires computing hash PREIMAGES (impossible!)
```

### Example

```
w = 16, message digits = [3, 7, 2, 10]

Checksum:
  C = (15-3) + (15-7) + (15-2) + (15-10)
    = 12 + 8 + 13 + 5
    = 38

Attacker wants to increase m₀ from 3 to 6:
  New message: [6, 7, 2, 10]
  New checksum: (15-6) + (15-7) + (15-2) + (15-10)
              = 9 + 8 + 13 + 5
              = 35

Checksum DECREASED from 38 to 35!
Attacker would need to forge a signature for checksum digit < 38.
But checksum signatures can only be hashed FORWARD (larger values).
Attack fails!
```

### WOTS+ Improvements

WOTS+ adds additional security features:
1. Bitmask (XOR with random values between hash iterations)
2. Proper parameter selection
3. Standardized in RFC 8391

For this project, we implement the core idea without the bitmask for clarity.

---

## The Checksum: Preventing Forgery {#checksum}

### Mathematical Analysis

Let the message be encoded as digits [m₀, m₁, ..., mₗ₁₋₁] where each mᵢ ∈ [0, w-1].

```
Checksum: C = Σᵢ (w - 1 - mᵢ)

Maximum message (all w-1): C = 0
Minimum message (all 0):   C = l₁ × (w - 1)
```

### Why the Checksum Works

```
Given a valid signature for message M with checksum C:

To forge message M' with any increased digit:
  At least one m'ᵢ > mᵢ
  Therefore C' < C

To sign checksum C' < C:
  Need to find preimage in checksum chains
  But attacker only has H^(c_j)(sk) for checksum digits c_j
  Computing H^(c'_j)(sk) where c'_j < c_j requires preimage

PREIMAGE IS IMPOSSIBLE → FORGERY IS IMPOSSIBLE
```

### Number of Checksum Chains

```
Maximum checksum: C_max = l₁ × (w - 1)

For w = 16, l₁ = 64:
  C_max = 64 × 15 = 960

Digits needed to encode 960 in base 16:
  ⌈log₁₆(960 + 1)⌉ = ⌈2.49⌉ = 3 digits

So l₂ = 3 checksum chains
```

---

## Complete WOTS+ Algorithm {#complete-algorithm}

### Parameters (This Project)

```
w = 16      (Winternitz parameter)
n = 32      (hash output size in bytes)
l₁ = 64     (message chains: 256 bits / 4 bits per chain)
l₂ = 3      (checksum chains)
l = 67      (total chains)
```

### Key Generation

```python
def keygen():
    # Generate l random seeds
    sk = [random_bytes(32) for _ in range(67)]

    # Compute public key by hashing each seed w-1 times
    pk = [chain_hash(sk[i], 15) for i in range(67)]

    return (sk, pk)
```

### Signing

```python
def sign(sk, message):
    # 1. Hash the message
    digest = hash(message)

    # 2. Convert to base-16 digits
    msg_digits = to_base_16(digest)  # 64 digits

    # 3. Compute checksum
    checksum = sum(15 - d for d in msg_digits)  # 0 to 960

    # 4. Encode checksum as base-16
    cs_digits = to_base_16_fixed(checksum, 3)  # 3 digits

    # 5. Combine all digits
    all_digits = msg_digits + cs_digits  # 67 digits

    # 6. Compute signature chains
    signature = []
    for i in range(67):
        sig_i = chain_hash(sk[i], all_digits[i])
        signature.append(sig_i)

    return signature
```

### Verification

```python
def verify(pk, message, signature):
    # 1. Hash the message
    digest = hash(message)

    # 2. Convert to base-16 digits
    msg_digits = to_base_16(digest)

    # 3. Compute checksum
    checksum = sum(15 - d for d in msg_digits)
    cs_digits = to_base_16_fixed(checksum, 3)
    all_digits = msg_digits + cs_digits

    # 4. Verify each chain
    for i in range(67):
        remaining = 15 - all_digits[i]
        computed = chain_hash(signature[i], remaining)
        if computed != pk[i]:
            return False

    return True
```

### Visual Walkthrough

```
Message: "Hello, World!"
         ↓
    Hash (SHA3-256)
         ↓
Digest: 0x3a7f... (32 bytes)
         ↓
    Base-16 encode
         ↓
Digits: [3, 10, 7, 15, ...] (64 digits, each 0-15)
         ↓
    Checksum: Σ(15 - dᵢ) = 487
         ↓
    Encode: [1, 14, 7] (3 digits)
         ↓
All digits: [3, 10, 7, 15, ..., 1, 14, 7] (67 total)
         ↓
    For each digit dᵢ, compute H^(dᵢ)(skᵢ)
         ↓
Signature: [H³(sk₀), H¹⁰(sk₁), H⁷(sk₂), ...] (67 × 32 bytes = 2144 bytes)
```

---

## Security Analysis {#security}

### Security Reduction

WOTS+ security reduces to hash function security:

```
If an attacker can forge WOTS+ signatures,
then they can either:
  1. Find hash preimages, OR
  2. Find hash collisions

Both are assumed to be computationally infeasible.
```

### Security Level

```
For n = 256 bit hash (Blake3 or SHA-256):

Classical security:
  - Preimage: 2^256 operations
  - Collision: 2^128 operations

Quantum security (Grover):
  - Preimage: 2^128 operations
  - Collision: 2^85 operations

Post-quantum security level: 128 bits ✓
```

### Why One-Time?

```
Signature reveals: H^(mᵢ)(skᵢ) for each chain

If you sign two messages M₁ and M₂:
  For chain i where m₁ᵢ < m₂ᵢ:
    First sig:  σ₁ᵢ = H^(m₁ᵢ)(skᵢ)
    Second sig: σ₂ᵢ = H^(m₂ᵢ)(skᵢ)

  Attacker learns: σ₂ᵢ = H^(m₂ᵢ - m₁ᵢ)(σ₁ᵢ)

  Can now forge any message with digit ≤ m₂ᵢ in position i!

NEVER REUSE A WOTS+ KEY
```

---

## Implementation in This Project {#implementation}

### Code Structure

```
src/wots/
├── mod.rs        # Parameters, error types, hash functions
├── keypair.rs    # WotsKeypair: generate(), sign()
└── signature.rs  # WotsSignature: verify(), serialization
```

### Key Data Structures

```rust
pub struct WotsKeypair {
    private_key: Vec<[u8; 32]>,  // 67 seeds
    public_key: Vec<[u8; 32]>,   // 67 endpoints
    used: bool,                   // One-time enforcement
    params: WotsParams,
}

pub struct WotsSignature {
    chains: Vec<[u8; 32]>,       // 67 chain values
    w: usize,                     // Winternitz parameter
    signed_digest: Vec<u8>,      // The digest that was signed
}
```

### One-Time Enforcement

```rust
pub fn sign(&mut self, digest: &[u8; 32]) -> Result<WotsSignature, WotsError> {
    if self.used {
        return Err(WotsError::KeyAlreadyUsed);
    }
    self.used = true;  // CRITICAL: Mark as used

    // ... proceed with signing
}
```

### Try It

```bash
cd quantum-winter-poc
cargo test test_one_time_property
cargo run  # See live demonstration
```

---

## Comparison with Other Schemes {#comparison}

### vs ECDSA

| Aspect | ECDSA | WOTS+ |
|--------|-------|-------|
| Key size | 32/33 bytes | 2144/2144 bytes |
| Signature size | 64 bytes | 2144 bytes |
| Sign time | ~100μs | ~200μs |
| Verify time | ~200μs | ~200μs |
| Quantum safe | **No** | **Yes** |
| Reusable | Yes | **No (one-time)** |

### vs SPHINCS+

| Aspect | WOTS+ | SPHINCS+ |
|--------|-------|----------|
| Key size | 2144 bytes | 32-64 bytes |
| Signature size | 2144 bytes | 8-49 KB |
| Quantum safe | Yes | Yes |
| Reusable | **No** | **Yes** |
| Complexity | Simple | Complex (Merkle trees) |

SPHINCS+ uses WOTS+ internally but adds Merkle trees to allow key reuse.

### vs Dilithium (NIST Standard)

| Aspect | WOTS+ | Dilithium |
|--------|-------|-----------|
| Basis | Hashes | Lattices |
| Key size | 2144 bytes | 1.3-2.5 KB |
| Signature size | 2144 bytes | 2.4-4.6 KB |
| Quantum safe | Yes | Yes |
| Reusable | No | Yes |
| Maturity | Decades | Years |

---

## Exercises {#exercises}

### Exercise 1: Checksum Calculation

For w=16 and message digits [5, 12, 0, 8], compute the checksum.

<details>
<summary>Solution</summary>

```
Checksum = Σ(15 - mᵢ)
         = (15-5) + (15-12) + (15-0) + (15-8)
         = 10 + 3 + 15 + 7
         = 35

Encoded in base-16: 35 = 2×16 + 3 = [2, 3]
```
</details>

### Exercise 2: Forgery Analysis

Why can't an attacker increase a message digit and decrease the corresponding checksum digit?

<details>
<summary>Solution</summary>

```
To increase message digit mᵢ from 5 to 8:
  Attacker has: σᵢ = H⁵(skᵢ)
  Attacker can compute: H⁸(skᵢ) = H³(σᵢ)  ✓ Easy!

But checksum decreased from 35 to 32.
  Attacker has checksum signature for digits [2, 3]
  Needs signature for digits [2, 0] (if encoding 32)

  For checksum chain j with digit decreasing from 3 to 0:
    Attacker has: σⱼ = H³(skⱼ)
    Needs: H⁰(skⱼ) = skⱼ

  This requires computing a PREIMAGE of H³!
  Computationally infeasible.
```
</details>

### Exercise 3: Parameter Selection

If we wanted 512-bit security, what parameters would we need?

<details>
<summary>Solution</summary>

```
For 512-bit hash output:

w = 16 (keep same)
n = 64 bytes (512 bits)
l₁ = 512/4 = 128 message chains
l₂ = ⌈log₁₆(128 × 15 + 1)⌉ = ⌈log₁₆(1921)⌉ = 3 checksum chains
l = 131 total chains

Signature size = 131 × 64 = 8,384 bytes

Trade-off: 4× larger signatures for 2× more security bits
(Though 256-bit is already overkill post-quantum)
```
</details>

### Exercise 4: Implementation

Trace through the verification of a signature where the message digit is 7 and the chain endpoint (public key) is pk.

<details>
<summary>Solution</summary>

```
Given:
  - Message digit m = 7
  - Signature component σ = H⁷(sk)
  - Public key component pk = H¹⁵(sk)
  - w = 16

Verification:
  1. Compute remaining hashes: 15 - 7 = 8
  2. Compute: H⁸(σ) = H⁸(H⁷(sk)) = H¹⁵(sk)
  3. Compare: H¹⁵(sk) =? pk
  4. Since pk = H¹⁵(sk) by construction, they match!
  5. Verification succeeds ✓
```
</details>

---

## Key Takeaways

1. **Hash-based signatures** derive security from hash function properties
2. **Lamport signatures** are simple but have huge keys/signatures
3. **Winternitz optimization** uses hash chains to reduce sizes
4. **The checksum** prevents forgery by making preimages necessary
5. **WOTS+ is quantum-resistant** with 128-bit post-quantum security
6. **One-time property is critical** - never reuse keys
7. **This project implements WOTS+** with w=16, n=32, producing 2144-byte signatures

---

## What's Next?

- **[07-ADVANCED-QUANTUM-ALGORITHMS](07-ADVANCED-QUANTUM-ALGORITHMS.md)** - Why WOTS+ survives quantum attacks
- **[08-ADVANCED-ZKPROOFS-STARKS](08-ADVANCED-ZKPROOFS-STARKS.md)** - How to compress many WOTS+ signatures
- **[09-PHD-CRYPTOGRAPHIC-SECURITY](09-PHD-CRYPTOGRAPHIC-SECURITY.md)** - Formal security proofs
