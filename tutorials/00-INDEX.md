# Tutorial Index: From Novice to Expert

Welcome to the comprehensive tutorial series for post-quantum cryptography and blockchain security. These tutorials are designed for learners at all levels, from complete beginners to PhD researchers.

---

## How to Use These Tutorials

### By Your Background

| Your Background | Start Here | Then Progress To |
|-----------------|------------|------------------|
| **Complete Novice** | 01-BEGINNER | 02-BEGINNER → 03-BEGINNER → Intermediate |
| **Math/Physics** | 01-BEGINNER (skim), then 04-INTERMEDIATE | 07-ADVANCED → 09-PHD |
| **Computer Science** | 02-BEGINNER, 04-INTERMEDIATE | 05-INTERMEDIATE → 06-INTERMEDIATE |
| **Finance/Economics** | 02-BEGINNER-BLOCKCHAIN | 05-INTERMEDIATE |
| **Software Developer** | 04-INTERMEDIATE | 05 → 06 → Implementation |
| **Cryptography Student** | 05-INTERMEDIATE | 07-ADVANCED → 09-PHD |
| **Quantum Physics** | 03-BEGINNER (blockchain context) | 07-ADVANCED |
| **PhD Researcher** | Skim Intermediate, focus on 09-PHD, 10-PHD | Research directions |

### By Topic

| Topic | Beginner | Intermediate | Advanced/PhD |
|-------|----------|--------------|--------------|
| **Cryptography Basics** | [01](01-BEGINNER-CRYPTOGRAPHY.md) | [04](04-INTERMEDIATE-HASH-FUNCTIONS.md) | [09](09-PHD-CRYPTOGRAPHIC-SECURITY.md) |
| **Blockchain** | [02](02-BEGINNER-BLOCKCHAIN.md) | [05](05-INTERMEDIATE-DIGITAL-SIGNATURES.md) | - |
| **Quantum Computing** | [03](03-BEGINNER-QUANTUM.md) | - | [07](07-ADVANCED-QUANTUM-ALGORITHMS.md) |
| **Hash Functions** | [01](01-BEGINNER-CRYPTOGRAPHY.md) | [04](04-INTERMEDIATE-HASH-FUNCTIONS.md) | [09](09-PHD-CRYPTOGRAPHIC-SECURITY.md) |
| **Digital Signatures** | [02](02-BEGINNER-BLOCKCHAIN.md) | [05](05-INTERMEDIATE-DIGITAL-SIGNATURES.md) | [09](09-PHD-CRYPTOGRAPHIC-SECURITY.md) |
| **WOTS+** | - | [06](06-INTERMEDIATE-WOTS.md) | [09](09-PHD-CRYPTOGRAPHIC-SECURITY.md) |
| **Zero-Knowledge Proofs** | - | - | [08](08-ADVANCED-ZKPROOFS-STARKS.md) |
| **Post-Quantum Crypto** | [03](03-BEGINNER-QUANTUM.md) | [06](06-INTERMEDIATE-WOTS.md) | [10](10-PHD-POST-QUANTUM-LANDSCAPE.md) |

---

## Tutorial List

### Beginner Level (No Prerequisites)

These tutorials assume no prior knowledge. Start here if you're new to any of these topics.

1. **[01-BEGINNER-CRYPTOGRAPHY](01-BEGINNER-CRYPTOGRAPHY.md)**
   - What is cryptography and why does it matter?
   - Secret codes through history
   - Modern cryptography concepts
   - Symmetric vs asymmetric encryption
   - Introduction to hash functions
   - *Time: 30-45 minutes*

2. **[02-BEGINNER-BLOCKCHAIN](02-BEGINNER-BLOCKCHAIN.md)**
   - What is a blockchain?
   - Bitcoin and Ethereum explained
   - Transactions and signatures
   - Wallets and addresses
   - Why security matters
   - *Time: 30-45 minutes*

3. **[03-BEGINNER-QUANTUM](03-BEGINNER-QUANTUM.md)**
   - What is quantum computing?
   - Qubits vs classical bits
   - Why quantum computers are powerful
   - The quantum threat to cryptography
   - Timeline and current state
   - *Time: 30-45 minutes*

### Intermediate Level (Some Technical Background)

These tutorials assume basic math (algebra) and some familiarity with computing concepts.

4. **[04-INTERMEDIATE-HASH-FUNCTIONS](04-INTERMEDIATE-HASH-FUNCTIONS.md)**
   - Mathematical definition of hash functions
   - Security properties (preimage, collision resistance)
   - Common hash functions (SHA-256, Blake3)
   - Hash chains and Merkle trees
   - Applications in cryptography
   - *Time: 45-60 minutes*

5. **[05-INTERMEDIATE-DIGITAL-SIGNATURES](05-INTERMEDIATE-DIGITAL-SIGNATURES.md)**
   - Mathematical foundations
   - RSA signatures
   - Elliptic curve cryptography (ECC)
   - ECDSA in Bitcoin and Ethereum
   - Why ECDSA is vulnerable to quantum attacks
   - *Time: 60-90 minutes*

6. **[06-INTERMEDIATE-WOTS](06-INTERMEDIATE-WOTS.md)**
   - One-time signatures concept
   - Lamport signatures
   - Winternitz improvement
   - WOTS+ detailed walkthrough
   - Implementation in this project
   - *Time: 60-90 minutes*

### Advanced Level (Strong Technical Background)

These tutorials assume comfort with mathematics and computer science fundamentals.

7. **[07-ADVANCED-QUANTUM-ALGORITHMS](07-ADVANCED-QUANTUM-ALGORITHMS.md)**
   - Quantum mechanics primer
   - Quantum gates and circuits
   - Shor's algorithm (detailed)
   - Grover's algorithm (detailed)
   - Impact on cryptographic primitives
   - *Time: 2-3 hours*

8. **[08-ADVANCED-ZKPROOFS-STARKS](08-ADVANCED-ZKPROOFS-STARKS.md)**
   - Zero-knowledge proofs fundamentals
   - Interactive vs non-interactive proofs
   - SNARKs vs STARKs
   - Algebraic Intermediate Representation (AIR)
   - STARK construction and verification
   - Application to signature aggregation
   - *Time: 2-3 hours*

### PhD Level (Research Depth)

These tutorials provide research-level depth suitable for graduate students and researchers.

9. **[09-PHD-CRYPTOGRAPHIC-SECURITY](09-PHD-CRYPTOGRAPHIC-SECURITY.md)**
   - Foundations of provable security
   - Security models and definitions (EUF-CMA)
   - Random oracle model and QROM
   - Hash function security properties
   - WOTS+ formal security proof
   - Tightness and reduction quality
   - Post-quantum security models
   - *Time: 4-6 hours*

10. **[10-PHD-POST-QUANTUM-LANDSCAPE](10-PHD-POST-QUANTUM-LANDSCAPE.md)**
    - NIST post-quantum standards (FIPS 203/204/205)
    - Lattice-based cryptography (LWE, Kyber, Dilithium)
    - Hash-based signatures (SPHINCS+, XMSS)
    - Code-based cryptography (McEliece, BIKE, HQC)
    - Multivariate cryptography (Oil-and-Vinegar)
    - Isogeny-based cryptography (CSIDH, SQISign)
    - Hybrid approaches and migration strategies
    - Research frontiers and open problems
    - *Time: 4-6 hours*

---

## Learning Paths

### Path 1: "I want to understand the quantum threat" (4-6 hours)

```
01-BEGINNER-CRYPTOGRAPHY
        ↓
03-BEGINNER-QUANTUM
        ↓
05-INTERMEDIATE-DIGITAL-SIGNATURES (focus on ECDSA section)
        ↓
07-ADVANCED-QUANTUM-ALGORITHMS
```

### Path 2: "I want to understand this project's code" (3-4 hours)

```
01-BEGINNER-CRYPTOGRAPHY
        ↓
04-INTERMEDIATE-HASH-FUNCTIONS
        ↓
06-INTERMEDIATE-WOTS
        ↓
(Read the code with docs/ARCHITECTURE.md)
```

### Path 3: "I want to understand blockchain security" (4-5 hours)

```
02-BEGINNER-BLOCKCHAIN
        ↓
05-INTERMEDIATE-DIGITAL-SIGNATURES
        ↓
03-BEGINNER-QUANTUM
        ↓
06-INTERMEDIATE-WOTS
```

### Path 4: "I'm a researcher exploring post-quantum crypto" (6-8 hours)

```
06-INTERMEDIATE-WOTS (review)
        ↓
07-ADVANCED-QUANTUM-ALGORITHMS
        ↓
09-PHD-CRYPTOGRAPHIC-SECURITY
        ↓
10-PHD-POST-QUANTUM-LANDSCAPE
        ↓
08-ADVANCED-ZKPROOFS-STARKS
```

### Path 5: "I'm a math/physics person new to crypto" (5-6 hours)

```
01-BEGINNER-CRYPTOGRAPHY (fast)
        ↓
04-INTERMEDIATE-HASH-FUNCTIONS
        ↓
05-INTERMEDIATE-DIGITAL-SIGNATURES
        ↓
07-ADVANCED-QUANTUM-ALGORITHMS (you'll excel here!)
        ↓
09-PHD-CRYPTOGRAPHIC-SECURITY
```

---

## Prerequisites Summary

| Tutorial | Math | CS | Crypto | Physics |
|----------|------|----|---------| --------|
| 01-BEGINNER-CRYPTO | None | None | None | None |
| 02-BEGINNER-BLOCKCHAIN | None | None | None | None |
| 03-BEGINNER-QUANTUM | None | None | None | None |
| 04-INTERMEDIATE-HASH | Algebra | Basic | 01 helpful | None |
| 05-INTERMEDIATE-SIGNATURES | Algebra, Modular | Basic | 01, 04 | None |
| 06-INTERMEDIATE-WOTS | Algebra | Basic | 04, 05 | None |
| 07-ADVANCED-QUANTUM | Linear Algebra | Algorithms | 05 | Helpful |
| 08-ADVANCED-ZKPROOFS | Abstract Algebra | Algorithms | 04, 05 | None |
| 09-PHD-SECURITY | Probability, Algebra | Complexity | All intermediate | None |
| 10-PHD-LANDSCAPE | Varies by topic | Varies | All | Varies |

---

## Notation Guide

Throughout these tutorials, we use consistent notation:

| Symbol | Meaning |
|--------|---------|
| H(x) | Hash function applied to x |
| H^n(x) | Hash function applied n times |
| \|\| | Concatenation |
| ⊕ | XOR (exclusive or) |
| mod n | Modular arithmetic |
| Z_n | Integers modulo n |
| G | Generator point (elliptic curves) |
| p, q | Prime numbers |
| n | Security parameter (often 256 bits) |
| λ | Security level |
| negl(λ) | Negligible function |
| PPT | Probabilistic Polynomial Time |
| |x| | Length/size of x |

---

## Getting Help

If you get stuck:

1. **Re-read the prerequisites** - You might need an earlier tutorial
2. **Check the glossary** - Each tutorial has key terms defined
3. **Work through examples** - Don't skip the worked examples
4. **Run the code** - This project has working implementations
5. **Take breaks** - Complex material needs time to absorb

---

## Contributing

Found an error? Have a suggestion? These tutorials welcome improvements:

1. Mathematical corrections
2. Clearer explanations
3. Additional examples
4. New topics
5. Translations

---

*Total estimated time to complete all tutorials: 25-35 hours*

*Remember: Understanding takes time. It's better to deeply understand fewer topics than to superficially skim all of them.*
