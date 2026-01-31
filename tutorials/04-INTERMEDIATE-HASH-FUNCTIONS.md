# Intermediate Tutorial: Hash Functions Deep Dive

**Time:** 45-60 minutes
**Prerequisites:** Basic cryptography concepts (Tutorial 01), basic algebra
**Goal:** Understand hash functions mathematically and their role in cryptography

---

## Table of Contents

1. [Formal Definition](#formal-definition)
2. [Security Properties](#security-properties)
3. [The Birthday Paradox](#birthday-paradox)
4. [Hash Function Constructions](#constructions)
5. [Common Hash Functions](#common-hashes)
6. [Hash Chains](#hash-chains)
7. [Merkle Trees](#merkle-trees)
8. [Applications in Cryptography](#applications)
9. [Quantum Impact on Hashes](#quantum-impact)
10. [Exercises](#exercises)

---

## Formal Definition {#formal-definition}

### Mathematical Definition

A **cryptographic hash function** H is a function:

```
H: {0,1}* → {0,1}^n
```

Where:
- `{0,1}*` = all binary strings of any length (the domain)
- `{0,1}^n` = all binary strings of exactly n bits (the codomain)
- n is typically 256 or 512 bits

### In Plain English

```
Input:  Any data of any size (text, files, images...)
Output: A fixed-size "fingerprint" (hash, digest)
```

### Example

Using SHA-256 (n = 256):

```
H("Hello") =
  185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969

H("Hello!") =
  33b506bd543fd0cf3ef0d00b3e69c42e2fd19e46dc3a8b3c3d3be2e8c0d5b6c7

H("The entire text of War and Peace...") =
  [some 256-bit value]
```

All outputs are exactly 256 bits (64 hex characters), regardless of input size.

---

## Security Properties {#security-properties}

A cryptographic hash function must satisfy three security properties:

### Property 1: Preimage Resistance (One-Way)

```
Given:  h = H(m)
Find:   m

Should be computationally infeasible.
```

**Intuition:** Given a fingerprint, you can't reconstruct the original person.

**Formal:** For random y ∈ {0,1}^n, any PPT adversary A:
```
Pr[H(A(y)) = y] ≤ negl(n)
```

### Property 2: Second Preimage Resistance

```
Given:  m₁
Find:   m₂ ≠ m₁ such that H(m₁) = H(m₂)

Should be computationally infeasible.
```

**Intuition:** Given a document, you can't find a different document with the same hash.

### Property 3: Collision Resistance

```
Find: m₁ ≠ m₂ such that H(m₁) = H(m₂)

Should be computationally infeasible.
```

**Intuition:** You can't find ANY two different documents with the same hash.

### Relationship Between Properties

```
Collision Resistance ⟹ Second Preimage Resistance ⟹ Preimage Resistance
(strongest)                                         (weakest)

If collision resistance is broken, the others MAY still hold.
```

### Security Levels

For a hash with n-bit output:

| Property | Attack Complexity | For n=256 |
|----------|-------------------|-----------|
| Preimage | O(2^n) | 2^256 |
| Second Preimage | O(2^n) | 2^256 |
| Collision | O(2^(n/2)) | 2^128 |

Why collision is easier? Birthday paradox (next section).

---

## The Birthday Paradox {#birthday-paradox}

### The Problem

How many people need to be in a room for there to be a 50% chance that two share a birthday?

**Intuitive answer:** About 183 (half of 365)
**Actual answer:** Only 23!

### Why This Happens

With 23 people, there are C(23,2) = 253 pairs.
Each pair has a 1/365 chance of matching.
The cumulative probability exceeds 50%.

### Application to Hash Functions

For a hash with n-bit output:
- There are 2^n possible hash values
- After computing ~√(2^n) = 2^(n/2) hashes, expect a collision

```
n = 256 bits:
  Preimage attack: 2^256 operations
  Collision attack: 2^128 operations

2^128 is still astronomical (10^38), but much less than 2^256 (10^77)
```

### Birthday Attack on Signatures

```
Attacker:
1. Generate 2^(n/2) variations of legitimate document D₁
2. Generate 2^(n/2) variations of malicious document D₂
3. Find pair where H(D₁ᵢ) = H(D₂ⱼ)
4. Get signature on D₁ᵢ
5. Use signature for D₂ⱼ (same hash!)
```

This is why we need collision resistance, not just preimage resistance.

---

## Hash Function Constructions {#constructions}

### Merkle-Damgård Construction

Most classic hash functions (MD5, SHA-1, SHA-2) use this structure:

```
Message: M = m₁ || m₂ || m₃ || ... || mₖ (broken into blocks)

          IV     m₁      m₂      m₃           mₖ
           │      │       │       │            │
           ▼      ▼       ▼       ▼            ▼
         ┌───┐  ┌───┐   ┌───┐   ┌───┐       ┌───┐
    IV──▶│ f │─▶│ f │──▶│ f │──▶│ f │─ ... ─▶│ f │──▶ H(M)
         └───┘  └───┘   └───┘   └───┘       └───┘

f = compression function (fixed-size input → fixed-size output)
IV = initialization vector (constant)
```

**Security:** If f is collision-resistant, so is H.

**Problem:** Length extension attacks possible.

### Sponge Construction

Used in SHA-3 (Keccak):

```
State: r bits (rate) + c bits (capacity)

      Absorbing Phase              Squeezing Phase
      ───────────────              ────────────────

m₁    m₂    m₃                    output
 │     │     │                       ▲
 ▼     ▼     ▼                       │
┌─────────────────┐               ┌──┴──┐
│  r   │    c     │──▶ ...  ──▶   │  r  │
└─────────────────┘               └─────┘
        │                             │
        ▼                             ▼
      ┌───┐                        ┌───┐
      │ f │ (permutation)          │ f │
      └───┘                        └───┘
```

**Advantages:**
- Variable output length
- No length extension attacks
- Clean security proof

---

## Common Hash Functions {#common-hashes}

### MD5 (1992)

```
Output: 128 bits
Status: BROKEN (collisions found in 2004)
Usage:  DO NOT use for security
```

### SHA-1 (1995)

```
Output: 160 bits
Status: BROKEN (practical collision in 2017)
Usage:  Legacy only, avoid
```

### SHA-2 Family (2001)

```
SHA-224: 224 bits (truncated SHA-256)
SHA-256: 256 bits ← Most common
SHA-384: 384 bits (truncated SHA-512)
SHA-512: 512 bits

Status: Secure
Usage:  Bitcoin, TLS, many applications
```

### SHA-3/Keccak (2015)

```
SHA3-224: 224 bits
SHA3-256: 256 bits
SHA3-384: 384 bits
SHA3-512: 512 bits

Status: Secure, newest standard
Usage:  Ethereum (Keccak-256), modern applications
```

### Blake2/Blake3 (2012/2020)

```
Output: Variable (commonly 256 bits)
Status: Secure
Usage:  Very fast, good for applications needing speed

Blake3 is ~6x faster than SHA-256 on modern CPUs.
```

### Comparison

| Hash | Bits | Speed (MB/s) | Status |
|------|------|--------------|--------|
| MD5 | 128 | ~500 | Broken |
| SHA-1 | 160 | ~400 | Broken |
| SHA-256 | 256 | ~200 | Secure |
| SHA-3 | 256 | ~150 | Secure |
| Blake3 | 256 | ~1000+ | Secure |

---

## Hash Chains {#hash-chains}

### Definition

A **hash chain** is the repeated application of a hash function:

```
H¹(x) = H(x)
H²(x) = H(H(x))
H³(x) = H(H(H(x)))
...
Hⁿ(x) = H(Hⁿ⁻¹(x))
```

### Properties

1. **One-way:** Given Hⁿ(x), can't compute x or Hᵏ(x) for k < n
2. **Deterministic:** Same x always produces same chain
3. **Verifiable:** Given Hᵏ(x) and Hⁿ(x), can verify by hashing (n-k) times

### Visual Representation

```
x ──H──▶ H(x) ──H──▶ H²(x) ──H──▶ H³(x) ──H──▶ ... ──H──▶ Hⁿ(x)
seed                                                      endpoint
(secret)                                                  (public)
```

### Application: One-Time Passwords (S/Key)

```
Setup:
  Choose secret s
  Compute H¹⁰⁰(s) and store on server

Login 1: Provide H⁹⁹(s)
  Server: Verify H(H⁹⁹(s)) = H¹⁰⁰(s) ✓
  Server: Store H⁹⁹(s)

Login 2: Provide H⁹⁸(s)
  Server: Verify H(H⁹⁸(s)) = H⁹⁹(s) ✓
  Server: Store H⁹⁸(s)

...

An attacker who intercepts H⁹⁹(s) can't compute H⁹⁸(s) (preimage)
```

### Application: WOTS+ Signatures

WOTS+ uses hash chains for quantum-resistant signatures:

```
Private key: Random seed
Public key:  Hʷ⁻¹(seed)  (hash seed w-1 times)
Signature:   Hᵐ(seed)    (hash seed m times, where m comes from message)
Verification: Check that Hʷ⁻¹⁻ᵐ(signature) = public_key
```

---

## Merkle Trees {#merkle-trees}

### Definition

A **Merkle tree** is a binary tree where:
- Leaves contain data hashes
- Internal nodes contain hashes of their children

### Structure

```
                    Root Hash
                   H(H₁₂ || H₃₄)
                  /              \
               H₁₂                H₃₄
           H(H₁||H₂)          H(H₃||H₄)
            /    \              /    \
          H₁      H₂          H₃      H₄
         H(D₁)   H(D₂)       H(D₃)   H(D₄)
          │       │           │       │
         D₁      D₂          D₃      D₄
        (data)  (data)      (data)  (data)
```

### Properties

1. **Root hash commits to all data:** Change any leaf → root changes
2. **Efficient proofs:** Prove data is in tree with O(log n) hashes
3. **Efficient updates:** Update one leaf with O(log n) hashes

### Merkle Proof

To prove D₃ is in the tree, provide:
```
Proof = [H₄, H₁₂]

Verifier computes:
  H₃ = H(D₃)
  H₃₄ = H(H₃ || H₄)
  Root = H(H₁₂ || H₃₄)

Check: Computed root = Given root?
```

Only O(log n) hashes needed, regardless of tree size!

### Applications

| Application | Use of Merkle Tree |
|-------------|-------------------|
| Bitcoin | Transaction tree in each block |
| Git | File and directory hashing |
| Certificate Transparency | Append-only log of certificates |
| IPFS | Content addressing |

---

## Applications in Cryptography {#applications}

### 1. Password Storage

```
Never store: password
Always store: H(password || salt)

Verification:
  User enters password
  Compute H(password || salt)
  Compare to stored hash
```

Salt prevents rainbow table attacks.

### 2. Message Authentication Codes (HMAC)

```
HMAC(K, M) = H((K ⊕ opad) || H((K ⊕ ipad) || M))

Proves: Message came from someone with key K
```

### 3. Digital Signatures

```
Sign(SK, M):
  h = H(M)
  σ = Sign_internal(SK, h)

Verify(PK, M, σ):
  h = H(M)
  Verify_internal(PK, h, σ)
```

Hashing first is essential for security and efficiency.

### 4. Commitment Schemes

```
Commit(m, r):   c = H(m || r)      (commit to message m with randomness r)
Open(m, r, c):  Check H(m || r) = c

Properties:
  - Hiding: c reveals nothing about m
  - Binding: Can't open to different m' ≠ m
```

### 5. Proof of Work

```
Find nonce such that H(block || nonce) < target

Difficulty: Adjust target to require ~10 minutes
Security:   No shortcut; must try many nonces
```

---

## Quantum Impact on Hashes {#quantum-impact}

### Grover's Algorithm Effect

```
Classical preimage attack: O(2^n) operations
Quantum preimage attack:   O(2^(n/2)) operations

Classical collision attack: O(2^(n/2)) operations
Quantum collision attack:   O(2^(n/3)) operations
```

### Security Levels

| Hash Output | Classical Preimage | Quantum Preimage | Classical Collision | Quantum Collision |
|-------------|-------------------|------------------|--------------------|--------------------|
| 128 bits | 2^128 | 2^64 | 2^64 | 2^43 |
| 256 bits | 2^256 | 2^128 | 2^128 | 2^85 |
| 384 bits | 2^384 | 2^192 | 2^192 | 2^128 |

### Recommendation

```
For 128-bit post-quantum security:
  - Use 256-bit hash for preimage resistance
  - Use 384-bit hash for collision resistance

SHA-256: Still secure for signatures (preimage)
SHA-384: Recommended for collision-critical applications
```

### Why Hash-Based Crypto is Quantum-Safe

```
Hash-based signatures (WOTS+, SPHINCS+):
  - Security relies on hash preimage resistance
  - 256-bit hash → 128-bit post-quantum security
  - This is still excellent security!

ECDSA/RSA:
  - Security relies on discrete log / factoring
  - Shor's algorithm: exponential → polynomial
  - Completely broken!
```

---

## Exercises {#exercises}

### Exercise 1: Birthday Bound

How many SHA-256 hashes do you need to compute before expecting a collision?

<details>
<summary>Solution</summary>

```
SHA-256 output: 256 bits = 2^256 possible values
Birthday bound: √(2^256) = 2^128 hashes

About 2^128 ≈ 3.4 × 10^38 hashes

At 10 billion hashes/second, this takes:
  3.4 × 10^38 / 10^10 / (60×60×24×365) ≈ 10^21 years
```
</details>

### Exercise 2: Hash Chain Verification

Given:
- H⁵(x) = "abc123"
- H¹⁰(x) = "def456"

How do you verify these are consistent?

<details>
<summary>Solution</summary>

```
Compute H⁵(H⁵(x)) = H⁵("abc123")
If result equals "def456", they're consistent.

You're computing H^5 on the H^5 value,
which gives H^10 if they're from the same chain.
```
</details>

### Exercise 3: Merkle Proof Size

For a Merkle tree with 1 million leaves, how many hashes are in a proof?

<details>
<summary>Solution</summary>

```
n = 1,000,000 leaves
Height = ⌈log₂(1,000,000)⌉ = 20 levels

Proof size = 20 hashes (one per level)

Each hash = 32 bytes (SHA-256)
Total proof = 20 × 32 = 640 bytes

Regardless of data size, proof is tiny!
```
</details>

### Exercise 4: Preimage vs Collision

Why is collision resistance harder to achieve than preimage resistance?

<details>
<summary>Solution</summary>

```
Preimage: Given h, find m where H(m) = h
  - Must hit a SPECIFIC target
  - Probability per try: 1/2^n
  - Expected tries: 2^n

Collision: Find ANY m₁, m₂ where H(m₁) = H(m₂)
  - ANY match works
  - Birthday paradox applies
  - Expected tries: 2^(n/2)

The attacker has more flexibility in collision attacks,
making them easier (fewer tries needed).
```
</details>

---

## Key Takeaways

1. **Hash functions** map arbitrary input to fixed-size output
2. **Three properties:** Preimage resistance, second preimage resistance, collision resistance
3. **Birthday paradox:** Collisions are found in O(2^(n/2)), not O(2^n)
4. **Hash chains:** Repeated hashing, foundation of WOTS+
5. **Merkle trees:** Efficient proofs of inclusion
6. **Quantum impact:** Grover gives √ speedup, but hashes remain usable with larger sizes

---

## What's Next?

- **[05-INTERMEDIATE-DIGITAL-SIGNATURES](05-INTERMEDIATE-DIGITAL-SIGNATURES.md)** - How signatures use hashes
- **[06-INTERMEDIATE-WOTS](06-INTERMEDIATE-WOTS.md)** - Hash-based signatures in detail
- **[09-PHD-CRYPTOGRAPHIC-SECURITY](09-PHD-CRYPTOGRAPHIC-SECURITY.md)** - Formal security proofs
