# Beginner Tutorial: Quantum Computing and the Cryptographic Threat

**Time:** 30-45 minutes
**Prerequisites:** None (basic math helpful)
**Goal:** Understand what quantum computers are and why they threaten current cryptography

---

## Table of Contents

1. [What is Quantum Computing?](#what-is-quantum)
2. [Classical Bits vs Qubits](#bits-vs-qubits)
3. [Why Quantum Computers Are Powerful](#why-powerful)
4. [The Threat to Cryptography](#threat)
5. [Shor's Algorithm (Simplified)](#shors)
6. [Grover's Algorithm (Simplified)](#grovers)
7. [Timeline: When Should We Worry?](#timeline)
8. [Harvest Now, Decrypt Later](#harvest)
9. [The Solution: Post-Quantum Cryptography](#solution)
10. [Key Takeaways](#takeaways)

---

## What is Quantum Computing? {#what-is-quantum}

### Classical Computers

Your laptop, phone, and all computers today are **classical computers**. They process information using **bits**:

```
A bit is either 0 or 1. Nothing else.

8 bits = 1 byte
1 byte can represent: 00000000 to 11111111 (0 to 255)
```

Classical computers are incredibly fast at many things, but some problems remain impossibly hard even for supercomputers.

### Quantum Computers

**Quantum computers** use the strange rules of quantum physics to process information differently. They use **qubits** instead of bits.

```
A qubit can be:
  - 0
  - 1
  - BOTH 0 AND 1 AT THE SAME TIME (superposition)
```

This isn't science fiction—it's real physics, and companies like IBM, Google, and others are building quantum computers today.

### The Key Insight

Quantum computers aren't "faster" in the normal sense. They're a different kind of computer that can solve **certain specific problems** exponentially faster than classical computers.

**Good for quantum computers:**
- Breaking current encryption (Shor's algorithm)
- Searching unsorted data (Grover's algorithm)
- Simulating quantum physics
- Optimization problems

**Not better than classical:**
- Web browsing
- Video games
- Most everyday computing
- Problems without quantum algorithms

---

## Classical Bits vs Qubits {#bits-vs-qubits}

### Classical Bit

A bit is like a coin lying on a table:

```
Heads (1) or Tails (0)
Never both. Never in between.
```

### Qubit (Superposition)

A qubit is like a coin spinning in the air:

```
While spinning:
  - It's not heads
  - It's not tails
  - It's BOTH at once (superposition)

When you look (measure):
  - It becomes heads OR tails
  - Probability depends on how it was spinning
```

### Why Superposition is Powerful

**Classical: 3 bits**
```
Can be ONE of these 8 states at a time:
000, 001, 010, 011, 100, 101, 110, 111
```

**Quantum: 3 qubits**
```
Can be ALL 8 states at the same time!
Each state has a probability amplitude.
```

**50 qubits in superposition = 2^50 states simultaneously**
That's over 1,000,000,000,000,000 states!

### Entanglement

Two qubits can be **entangled**—their states become correlated:

```
If qubit A is measured as 0, qubit B is definitely 1
If qubit A is measured as 1, qubit B is definitely 0

This correlation is instant, regardless of distance!
(Einstein called it "spooky action at a distance")
```

Entanglement lets quantum computers perform coordinated operations across many qubits.

### The Catch: Measurement Collapses Superposition

```
Before measurement:
  Qubit is in superposition of 0 AND 1

After measurement:
  Qubit is definitely 0 OR definitely 1
  Superposition is destroyed
```

This is why quantum algorithms must be cleverly designed—you can't just "look" at all the parallel states.

---

## Why Quantum Computers Are Powerful {#why-powerful}

### Parallelism Through Superposition

Consider searching a phone book for a specific number:

**Classical approach:**
```
Check entry 1: No
Check entry 2: No
Check entry 3: No
...
Check entry N: Yes!

Time: O(N) - must check each entry
```

**Quantum approach (Grover's algorithm):**
```
Put all entries in superposition
Apply quantum operations
Amplify the correct answer
Measure

Time: O(√N) - square root of entries
```

For N = 1,000,000:
- Classical: 1,000,000 checks
- Quantum: ~1,000 checks

### Interference

Quantum states can **interfere** like waves:

```
Wave 1:    /\  /\  /\
Wave 2:    /\  /\  /\
───────────────────────
Sum:      /\/\/\/\/\/\  (constructive - bigger)

Wave 1:    /\  /\  /\
Wave 2:     \/  \/  \/
───────────────────────
Sum:       ──────────  (destructive - cancels out)
```

Quantum algorithms use interference to:
- Amplify correct answers (constructive)
- Cancel wrong answers (destructive)

---

## The Threat to Cryptography {#threat}

### What Quantum Computers Can Break

| Cryptography Type | Example | Quantum Impact |
|-------------------|---------|----------------|
| Symmetric encryption | AES-256 | Weakened (use larger keys) |
| Asymmetric encryption | RSA | **BROKEN** |
| Digital signatures | ECDSA | **BROKEN** |
| Hash functions | SHA-256 | Slightly weakened |

### Why RSA and ECDSA Are Broken

These algorithms are based on math problems that quantum computers can solve:

**RSA Security:**
```
Based on: Factoring large numbers is hard

N = p × q (where p and q are huge primes)

Classical: Factoring N takes mass-of-universe time
Quantum:   Shor's algorithm factors N in polynomial time
```

**ECDSA Security:**
```
Based on: Discrete logarithm problem is hard

Given: P (point on elliptic curve), Q = k×P (another point)
Find: k (the private key)

Classical: Finding k takes mass-of-universe time
Quantum:   Shor's algorithm finds k in polynomial time
```

### What "Broken" Means for You

```
Current Bitcoin/Ethereum:
  - Your private key is a secret number
  - Your public key is derived from it
  - When you spend, you reveal your public key
  - Quantum computer can derive your private key from public key
  - Attacker can steal all remaining funds at that address
```

---

## Shor's Algorithm (Simplified) {#shors}

### What It Does

Shor's Algorithm (1994) efficiently factors large numbers and solves discrete logarithm problems on quantum computers.

### Why It Matters

Most public-key cryptography (RSA, ECDSA, DH) relies on these problems being hard.

### Simplified Explanation

**Factoring 15 (classical):**
```
Try 2: 15/2 = 7.5 (not integer, not a factor)
Try 3: 15/3 = 5 (integer! 15 = 3 × 5)
```

**Factoring a 2048-bit number:**
```
Number of possible factors: ~10^300
Age of universe in seconds: ~10^17

Even checking one factor per second,
classical computers can't do it.
```

**Shor's Algorithm:**
```
1. Use superposition to try many factors simultaneously
2. Use quantum interference to find periodicity
3. Use periodicity to find factors
4. Result: Polynomial time instead of exponential
```

### The Impact

| Key Size | Classical Attack | Quantum Attack (Shor) |
|----------|------------------|----------------------|
| RSA-2048 | 10^40 years | Hours to days |
| ECDSA-256 | 10^40 years | Hours to days |

---

## Grover's Algorithm (Simplified) {#grovers}

### What It Does

Grover's Algorithm (1996) searches unsorted databases quadratically faster than classical computers.

### Simplified Explanation

**Searching for a needle in a haystack:**

```
Classical:
  - N items in haystack
  - Average: check N/2 items
  - Worst case: check all N items

Grover's:
  - Put all items in superposition
  - Amplify the "needle" state
  - Measure after √N iterations
  - Found in O(√N) time
```

### Impact on Symmetric Cryptography

```
AES-128:
  - Key space: 2^128 possibilities
  - Classical brute force: 2^128 operations
  - Grover: √(2^128) = 2^64 operations

Solution: Double the key size
  - AES-256 with Grover: 2^128 operations (still secure)
```

### Why Grover's Is Less Scary Than Shor's

| Algorithm | Impact | Solution |
|-----------|--------|----------|
| Shor's | Exponential → Polynomial | **Complete replacement needed** |
| Grover's | N → √N | Double key sizes |

Shor's completely breaks RSA/ECDSA.
Grover's just requires larger keys for symmetric crypto.

---

## Timeline: When Should We Worry? {#timeline}

### Current State (2026)

| Company | Qubits | Status |
|---------|--------|--------|
| IBM | 1,000+ | Noisy, error-prone |
| Google | 100+ | Demonstrated "quantum advantage" |
| IonQ | 30+ | Trapped ion approach |

**To break cryptography, we need:**
- Thousands of **error-corrected** qubits
- Current qubits are "noisy" (make mistakes)
- Error correction requires many physical qubits per logical qubit

### Expert Estimates for "Q-Day"

| Source | Estimate | Confidence |
|--------|----------|------------|
| Conservative | 2035-2040 | High |
| Moderate | 2030-2035 | Medium |
| Optimistic | 2028-2030 | Lower |
| NIST | "Plan for 2030" | Official guidance |

### Why Even Conservative Estimates Matter

```
If Q-Day is 2035...

Data encrypted today (2026):
  - Will still be secret in 2026
  - Will be decryptable in 2035

If data needs to be secret for 20 years:
  - Already vulnerable to "harvest now, decrypt later"
```

---

## Harvest Now, Decrypt Later {#harvest}

### The Strategy

Nation-state adversaries are likely doing this today:

```
2026:
  Adversary records:
    - Encrypted communications
    - Blockchain transactions
    - Public keys revealed in transactions

2035 (Q-Day):
  Adversary uses quantum computer to:
    - Decrypt stored communications
    - Derive private keys from public keys
    - Steal cryptocurrency from exposed addresses
```

### What's at Risk

**Already vulnerable (public key exposed):**
- $718 billion in Bitcoin at addresses that have sent transactions
- All Ethereum accounts that have ever transacted
- Historical encrypted communications

**Currently protected (public key not exposed):**
- Bitcoin addresses that have only received
- New addresses after post-quantum upgrade

### The Race

```
Good guys: Develop and deploy post-quantum cryptography
Bad guys:  Build quantum computers, harvest encrypted data

We need to win this race BEFORE Q-Day.
```

---

## The Solution: Post-Quantum Cryptography {#solution}

### What is Post-Quantum Cryptography?

Cryptographic algorithms designed to resist both classical AND quantum attacks.

### Main Approaches

| Approach | Based On | Example | Status |
|----------|----------|---------|--------|
| **Lattice-based** | Hard lattice problems | CRYSTALS-Kyber | NIST standard |
| **Hash-based** | Hash function security | SPHINCS+, WOTS+ | NIST standard |
| **Code-based** | Error-correcting codes | McEliece | Long-studied |
| **Multivariate** | Polynomial systems | Rainbow | Some broken |
| **Isogeny-based** | Elliptic curve isogenies | SIKE | Broken in 2022 |

### Why Hash-Based Signatures Are Promising

```
Security of ECDSA:
  Based on: Discrete log problem (broken by Shor)

Security of WOTS+ (hash-based):
  Based on: Hash function preimage resistance

Grover's impact on hashes:
  256-bit hash → 128-bit security (still excellent!)
```

**Hash-based signatures are the most conservative choice** because their security is well-understood and relies only on hash functions.

### WOTS+ (This Project)

WOTS+ (Winternitz One-Time Signatures Plus) is:
- Based only on hash functions
- Quantum-resistant
- Well-understood security
- Trade-off: Larger signatures, one-time use

This project implements WOTS+ to demonstrate post-quantum signatures.

### Industry Response

**NIST (August 2024):**
- Released FIPS 203, 204, 205
- Standardized Kyber, Dilithium, SPHINCS+

**Ethereum Foundation (January 2026):**
- Post-quantum security "top priority"
- $2M in prizes for PQ research
- New post-quantum team formed

---

## Key Takeaways {#takeaways}

1. **Quantum computers** use qubits that can be 0 AND 1 simultaneously

2. **Shor's Algorithm** breaks RSA and ECDSA (exponential speedup)

3. **Grover's Algorithm** weakens symmetric crypto (quadratic speedup)

4. **Q-Day estimates:** 2030-2040, but "harvest now, decrypt later" is happening today

5. **$718 billion** in Bitcoin at addresses with exposed public keys

6. **Post-quantum cryptography** is being standardized (NIST FIPS 203-205)

7. **Hash-based signatures** (like WOTS+) are quantum-resistant

8. **The time to prepare is NOW**, not when quantum computers arrive

---

## Glossary

| Term | Definition |
|------|------------|
| **Qubit** | Quantum bit, can be in superposition |
| **Superposition** | Being in multiple states simultaneously |
| **Entanglement** | Correlated quantum states |
| **Measurement** | Observing a qubit (collapses superposition) |
| **Shor's Algorithm** | Quantum algorithm breaking RSA/ECDSA |
| **Grover's Algorithm** | Quantum search algorithm (√N speedup) |
| **Q-Day** | Day quantum computers break cryptography |
| **HNDL** | Harvest Now, Decrypt Later |
| **Post-Quantum** | Cryptography secure against quantum attacks |
| **NIST** | National Institute of Standards and Technology |

---

## What's Next?

- **[04-INTERMEDIATE-HASH-FUNCTIONS](04-INTERMEDIATE-HASH-FUNCTIONS.md)** - Deep dive into hash functions
- **[06-INTERMEDIATE-WOTS](06-INTERMEDIATE-WOTS.md)** - Learn how WOTS+ works
- **[07-ADVANCED-QUANTUM-ALGORITHMS](07-ADVANCED-QUANTUM-ALGORITHMS.md)** - Detailed Shor's and Grover's

---

## Self-Check Questions

1. What's the difference between a bit and a qubit?
2. Why can't you just "look" at all superposition states?
3. What does Shor's algorithm break?
4. What does Grover's algorithm affect?
5. What is "harvest now, decrypt later"?
6. Why are hash-based signatures quantum-resistant?

*If you can explain these concepts, you understand the quantum threat!*
