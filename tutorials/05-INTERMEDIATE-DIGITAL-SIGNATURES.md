# Intermediate Tutorial: Digital Signatures and ECDSA

**Time:** 60-90 minutes
**Prerequisites:** Tutorial 01 (Cryptography), Tutorial 04 (Hash Functions), basic algebra
**Goal:** Understand how digital signatures work, especially ECDSA used in Bitcoin/Ethereum

---

## Table of Contents

1. [What Are Digital Signatures?](#what-are-signatures)
2. [Mathematical Foundation](#math-foundation)
3. [RSA Signatures](#rsa)
4. [Elliptic Curve Cryptography Basics](#ecc-basics)
5. [ECDSA: The Algorithm](#ecdsa)
6. [ECDSA in Bitcoin and Ethereum](#ecdsa-blockchain)
7. [Why ECDSA is Quantum Vulnerable](#quantum-vulnerable)
8. [Signature Schemes Comparison](#comparison)
9. [Exercises](#exercises)

---

## What Are Digital Signatures? {#what-are-signatures}

### Purpose

A digital signature provides:
1. **Authentication:** Proves the signer's identity
2. **Integrity:** Proves the message wasn't modified
3. **Non-repudiation:** Signer can't deny signing

### Formal Definition

A signature scheme consists of three algorithms:

```
KeyGen() → (pk, sk)
  Generate public key pk and secret key sk

Sign(sk, m) → σ
  Create signature σ on message m using secret key sk

Verify(pk, m, σ) → {true, false}
  Verify signature σ on message m using public key pk
```

### Security Requirement

**Existential Unforgeability under Chosen Message Attack (EUF-CMA):**

```
Attacker can:
  - See public key pk
  - Request signatures on any messages of their choice

Attacker cannot:
  - Forge a valid signature on any NEW message
  (even with unlimited computational power... classically)
```

---

## Mathematical Foundation {#math-foundation}

### Modular Arithmetic

```
a ≡ b (mod n) means: (a - b) is divisible by n

Examples:
  17 ≡ 5 (mod 12)   because 17 - 5 = 12
  25 ≡ 1 (mod 12)   because 25 - 1 = 24 = 2×12
```

**Modular exponentiation:**
```
3⁴ mod 7 = 81 mod 7 = 4

Used heavily in RSA
```

### Groups

A **group** (G, ·) is a set with an operation satisfying:
1. Closure: a · b ∈ G
2. Associativity: (a · b) · c = a · (b · c)
3. Identity: ∃ e such that a · e = a
4. Inverse: ∀ a, ∃ a⁻¹ such that a · a⁻¹ = e

**Examples:**
```
(ℤ_n*, ×) = integers 1 to n-1 coprime to n, under multiplication mod n
Used in RSA

Elliptic curve points under point addition
Used in ECDSA
```

### The Discrete Logarithm Problem

```
Given: g, h where h = g^x (in some group)
Find:  x

Easy direction: Given g and x, compute g^x
Hard direction: Given g and g^x, find x
```

This is the foundation of:
- Diffie-Hellman key exchange
- DSA signatures
- ECDSA signatures

---

## RSA Signatures {#rsa}

### Key Generation

```
1. Choose large primes p, q (each ~1024 bits)
2. Compute n = p × q
3. Compute φ(n) = (p-1)(q-1)
4. Choose e coprime to φ(n) (commonly e = 65537)
5. Compute d = e⁻¹ mod φ(n)

Public key:  (n, e)
Private key: (n, d)
```

### Signing

```
Sign(d, m):
  h = H(m)              # Hash the message
  σ = h^d mod n         # "Decrypt" the hash with private key
  return σ

This works because only the private key holder can compute h^d.
```

### Verification

```
Verify(e, m, σ):
  h = H(m)              # Hash the message
  h' = σ^e mod n        # "Encrypt" the signature with public key
  return (h == h')

Why this works:
  σ^e = (h^d)^e = h^(d×e) = h^1 = h (mod n)
  Because d×e ≡ 1 (mod φ(n))
```

### RSA Security

```
Security based on: Factoring n = p × q is hard

Classical attack: Exponential time
Quantum attack:   Polynomial time (Shor's algorithm)

RSA IS BROKEN BY QUANTUM COMPUTERS
```

---

## Elliptic Curve Cryptography Basics {#ecc-basics}

### What is an Elliptic Curve?

An elliptic curve is defined by the equation:

```
y² = x³ + ax + b

Where 4a³ + 27b² ≠ 0 (ensures no singularities)
```

### Visual Representation

```
        │
     ●──┼──●
    /   │   \
   ●    │    ●
  /     │     \
 ●      │      ●
────────┼──────────
        │
        │
```

The curve is symmetric about the x-axis.

### Point Addition

Given two points P and Q on the curve, we can define P + Q:

```
1. Draw a line through P and Q
2. The line intersects the curve at a third point R'
3. Reflect R' over the x-axis to get R = P + Q
```

```
         Q ●
          \
           \
        P ● \
             \
              ● R' (intersection)
              │
              │ (reflect)
              │
              ● R = P + Q
```

### Scalar Multiplication

```
k × P = P + P + P + ... + P (k times)

Example:
  3 × P = P + P + P

This is computed efficiently using "double-and-add":
  2P = P + P
  3P = 2P + P
```

### The Elliptic Curve Discrete Log Problem (ECDLP)

```
Given: P (base point), Q = k × P (another point)
Find:  k

Easy:  Given k and P, compute Q = k × P
Hard:  Given P and Q, find k
```

### Why Elliptic Curves?

Same security as RSA with much smaller keys:

| Security | RSA Key Size | ECC Key Size |
|----------|--------------|--------------|
| 80 bits | 1024 bits | 160 bits |
| 128 bits | 3072 bits | 256 bits |
| 256 bits | 15360 bits | 512 bits |

ECC keys are ~10x smaller for equivalent security!

### The secp256k1 Curve (Bitcoin/Ethereum)

```
Equation: y² = x³ + 7 (over finite field F_p)
p = 2²⁵⁶ - 2³² - 977 (a large prime)
Order n = number of points ≈ 2²⁵⁶

Generator point G is a specific point on the curve.
```

---

## ECDSA: The Algorithm {#ecdsa}

### Key Generation

```
1. Choose random integer d ∈ [1, n-1]  (private key)
2. Compute Q = d × G                    (public key)

Private key: d (256-bit integer)
Public key:  Q (point on curve, ~512 bits uncompressed, ~257 bits compressed)
```

### Signing

```
Sign(d, m):
  1. Compute e = H(m)                     # Hash the message
  2. Choose random k ∈ [1, n-1]           # Ephemeral key (CRITICAL!)
  3. Compute (x, y) = k × G               # Point multiplication
  4. Compute r = x mod n                  # x-coordinate mod n
  5. Compute s = k⁻¹ × (e + r × d) mod n  # The signature equation
  6. Return (r, s)
```

### Verification

```
Verify(Q, m, (r, s)):
  1. Compute e = H(m)
  2. Compute w = s⁻¹ mod n
  3. Compute u₁ = e × w mod n
  4. Compute u₂ = r × w mod n
  5. Compute (x, y) = u₁ × G + u₂ × Q     # Point arithmetic
  6. Return (r == x mod n)
```

### Why Verification Works

```
The signature equation:
  s = k⁻¹(e + rd) mod n

Rearranging:
  k = s⁻¹(e + rd) = s⁻¹e + s⁻¹rd = we + wrd mod n

So:
  k × G = (we + wrd) × G
        = we × G + wrd × G
        = we × G + wr × (d × G)
        = u₁ × G + u₂ × Q

The x-coordinate of k × G should equal r (by construction in signing).
```

### Critical Security: The Ephemeral Key k

```
WARNING: If k is ever reused or predictable, private key d is revealed!

If same k used for two signatures (r₁,s₁) and (r₂,s₂) on messages m₁, m₂:
  s₁ = k⁻¹(e₁ + r₁d)
  s₂ = k⁻¹(e₂ + r₂d)

  Since r₁ = r₂ = r (same k means same point):
  s₁ - s₂ = k⁻¹(e₁ - e₂)
  k = (e₁ - e₂) / (s₁ - s₂)

  Then: d = (s₁k - e₁) / r₁

Private key recovered!

This happened in the PlayStation 3 hack (Sony used constant k)
and in early Bitcoin implementations.
```

---

## ECDSA in Bitcoin and Ethereum {#ecdsa-blockchain}

### Bitcoin Transaction Signing

```
1. Create transaction:
   TX = {inputs, outputs, amounts, ...}

2. Compute transaction hash:
   h = SHA256(SHA256(TX))

3. Sign with private key:
   (r, s) = ECDSA_Sign(private_key, h)

4. Attach signature and public key:
   Signed_TX = TX + signature + public_key

5. Broadcast to network
```

### Bitcoin Signature Format

```
DER encoding (variable length, typically 71-72 bytes):
  0x30 [length] 0x02 [r_length] [r] 0x02 [s_length] [s]

Example:
  30 45 02 21 00 [r: 33 bytes] 02 20 [s: 32 bytes]
```

### Ethereum Differences

```
1. Uses Keccak-256 instead of SHA-256
2. Signature includes 'v' (recovery id) to recover public key
3. Signature format: (r, s, v) where v ∈ {27, 28}

Ethereum signature: 65 bytes total
  r: 32 bytes
  s: 32 bytes
  v: 1 byte
```

### Public Key Recovery

Ethereum uses signature recovery to avoid transmitting the full public key:

```
Given: message m, signature (r, s, v)
Recover: public key Q

This is possible because r determines (at most) 2 candidate points,
and v specifies which one.

Saves bandwidth: 65 bytes instead of 65 + 33 = 98 bytes
```

---

## Why ECDSA is Quantum Vulnerable {#quantum-vulnerable}

### The Attack

Shor's algorithm solves the Elliptic Curve Discrete Log Problem:

```
Given: Q = d × G
Find:  d

Classical: O(2^(n/2)) operations (infeasible for n=256)
Quantum:   O(n³) operations (polynomial time!)
```

### Impact on Blockchain

```
Scenario:
1. Alice's public key Q is revealed when she spends
2. Quantum attacker sees Q
3. Attacker computes d = discrete_log(Q, G)
4. Attacker can now sign transactions as Alice
5. Attacker steals all remaining funds at Alice's address
```

### What's Vulnerable

| Component | Quantum Impact |
|-----------|----------------|
| Private keys | Can be derived from public keys |
| Addresses (hashed) | Protected until first spend |
| Past transactions | Immutable, but addresses exposed |
| Future funds | Must move to quantum-safe addresses |

### The "Harvest Now, Decrypt Later" Threat

```
2026: Attacker records all public keys from transactions
2035: Attacker uses quantum computer to derive private keys
2035: Attacker steals from addresses that still have funds
```

**At risk:** ~$718 billion in Bitcoin at addresses with exposed public keys

---

## Signature Schemes Comparison {#comparison}

### Overview

| Scheme | Basis | Key Size | Sig Size | Quantum Safe? |
|--------|-------|----------|----------|---------------|
| RSA-2048 | Factoring | 2048 bits | 2048 bits | No |
| ECDSA-256 | ECDLP | 256 bits | 512 bits | No |
| Ed25519 | ECDLP | 256 bits | 512 bits | No |
| WOTS+ | Hash chains | 2144 bytes | 2144 bytes | **Yes** |
| SPHINCS+ | Hash trees | 32-64 bytes | 8-49 KB | **Yes** |
| Dilithium | Lattices | 1.3-2.5 KB | 2.4-4.6 KB | **Yes** |

### Performance (Approximate)

| Scheme | KeyGen | Sign | Verify |
|--------|--------|------|--------|
| RSA-2048 | 100ms | 1ms | 0.05ms |
| ECDSA-256 | 0.1ms | 0.1ms | 0.2ms |
| WOTS+ | 2ms | 0.2ms | 0.2ms |
| SPHINCS+-128f | 1ms | 5ms | 0.5ms |
| Dilithium2 | 0.1ms | 0.3ms | 0.1ms |

### Trade-offs

```
ECDSA:
  ✓ Small keys and signatures
  ✓ Fast operations
  ✗ Quantum vulnerable

WOTS+:
  ✓ Quantum safe
  ✓ Simple, well-understood
  ✗ Large signatures (~2KB)
  ✗ One-time use only

SPHINCS+:
  ✓ Quantum safe
  ✓ Stateless (reusable keys)
  ✗ Large signatures (8-49KB)
  ✗ Slower signing

Dilithium:
  ✓ Quantum safe
  ✓ Reasonable sizes
  ✓ Fast operations
  ? Newer, less studied
```

---

## Exercises {#exercises}

### Exercise 1: ECDSA Security

Why must the ephemeral key k be random for each signature?

<details>
<summary>Solution</summary>

```
If k is reused for two messages m₁ and m₂:

s₁ = k⁻¹(H(m₁) + r·d) mod n
s₂ = k⁻¹(H(m₂) + r·d) mod n

Subtracting:
s₁ - s₂ = k⁻¹(H(m₁) - H(m₂)) mod n

Therefore:
k = (H(m₁) - H(m₂)) / (s₁ - s₂) mod n

Once k is known:
d = (s₁·k - H(m₁)) / r mod n

The private key is completely compromised!
```
</details>

### Exercise 2: Key Sizes

Why does ECC use smaller keys than RSA for equivalent security?

<details>
<summary>Solution</summary>

```
RSA security: Based on integer factorization
  Best known attack: General Number Field Sieve
  Complexity: O(exp(c · n^(1/3) · (log n)^(2/3)))
  For 128-bit security: Need n ≈ 3072 bits

ECC security: Based on elliptic curve discrete log
  Best known attack: Pollard's rho
  Complexity: O(√n) where n is the group order
  For 128-bit security: Need n ≈ 256 bits

The ECDLP is harder to solve relative to key size,
allowing much smaller keys for the same security level.
```
</details>

### Exercise 3: Signature Verification

In ECDSA verification, we compute u₁×G + u₂×Q. Why does this work?

<details>
<summary>Solution</summary>

```
From signing: s = k⁻¹(e + rd) mod n
Rearranging: k = s⁻¹e + s⁻¹rd mod n
            k = we + wrd mod n    (where w = s⁻¹)

Multiplying both sides by G:
  kG = (we)G + (wrd)G
  kG = (we)G + (wr)(dG)
  kG = u₁G + u₂Q        (since Q = dG)

The point kG has x-coordinate r (by construction).
So if the verification passes, the signature is valid.
```
</details>

### Exercise 4: Quantum Threat

How long would it take a quantum computer to break a 256-bit ECDSA key?

<details>
<summary>Solution</summary>

```
Shor's algorithm for ECDLP: O(n³) quantum operations
For n = 256 bits: O(256³) ≈ O(2²⁴) ≈ 16 million operations

With error correction overhead, realistic estimates:
  - Physical qubits needed: ~2000-4000
  - Time: Hours to days

Compare to classical attack:
  - Pollard's rho: O(2^128) operations
  - Time: Longer than age of universe

Quantum speedup: From infeasible to hours/days
```
</details>

---

## Key Takeaways

1. **Digital signatures** provide authentication, integrity, and non-repudiation

2. **RSA** uses factoring; **ECDSA** uses elliptic curve discrete log

3. **ECDSA** has much smaller keys than RSA (256 bits vs 3072 bits)

4. **The ephemeral key k in ECDSA must be random** - reuse reveals private key

5. **Bitcoin uses secp256k1; Ethereum uses secp256k1 with Keccak-256**

6. **Quantum computers break ECDSA** using Shor's algorithm

7. **Post-quantum alternatives:** WOTS+ (this project), SPHINCS+, Dilithium

---

## What's Next?

- **[06-INTERMEDIATE-WOTS](06-INTERMEDIATE-WOTS.md)** - Quantum-safe signatures using hash chains
- **[07-ADVANCED-QUANTUM-ALGORITHMS](07-ADVANCED-QUANTUM-ALGORITHMS.md)** - How Shor's algorithm works
- **[09-PHD-CRYPTOGRAPHIC-SECURITY](09-PHD-CRYPTOGRAPHIC-SECURITY.md)** - Formal security proofs
