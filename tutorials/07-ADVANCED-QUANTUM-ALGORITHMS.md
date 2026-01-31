# Advanced Tutorial: Quantum Algorithms (Shor's and Grover's)

**Time:** 2-3 hours
**Prerequisites:** Linear algebra, basic quantum mechanics concepts helpful
**Goal:** Understand how quantum algorithms break cryptography and what remains secure

---

## Table of Contents

1. [Quantum Mechanics Primer](#quantum-primer)
2. [Quantum Computing Model](#quantum-model)
3. [Quantum Gates](#quantum-gates)
4. [Shor's Algorithm](#shors-algorithm)
5. [Grover's Algorithm](#grovers-algorithm)
6. [Impact on Cryptography](#impact)
7. [What Survives Quantum Attacks](#survivors)
8. [Current State of Quantum Computers](#current-state)

---

## Quantum Mechanics Primer {#quantum-primer}

### The Double-Slit Experiment

When particles (photons, electrons) pass through two slits, they create an interference pattern:

```
Single particle behavior:
  - Goes through "both slits at once"
  - Interferes with itself
  - Creates wave-like pattern on detector

        Slits      Detector
Source    │ │      ████
    ●─────│ │──────████████
          │ │      ████
                   ████████
                   ████
```

This demonstrates **superposition**: a particle can be in multiple states simultaneously until measured.

### Superposition

A quantum state can be a combination of classical states:

```
Classical bit:  0 or 1 (never both)

Qubit: |ψ⟩ = α|0⟩ + β|1⟩

Where:
  |0⟩, |1⟩ are basis states
  α, β are complex numbers (amplitudes)
  |α|² + |β|² = 1 (probabilities sum to 1)
```

When measured, the qubit collapses:
- Probability of measuring 0: |α|²
- Probability of measuring 1: |β|²

### Dirac Notation (Bra-Ket)

```
|ψ⟩ = "ket psi" (column vector)
⟨ψ| = "bra psi" (row vector, conjugate transpose)
⟨φ|ψ⟩ = inner product (scalar)
|φ⟩⟨ψ| = outer product (matrix)
```

### Common States

```
|0⟩ = [1, 0]ᵀ   (classical 0)
|1⟩ = [0, 1]ᵀ   (classical 1)

|+⟩ = (|0⟩ + |1⟩)/√2   (equal superposition)
|−⟩ = (|0⟩ − |1⟩)/√2   (equal superposition, opposite phase)
```

### Entanglement

Two qubits can be entangled—their states are correlated:

```
Bell state: |Φ⁺⟩ = (|00⟩ + |11⟩)/√2

Meaning:
  - If first qubit measured as 0, second is definitely 0
  - If first qubit measured as 1, second is definitely 1
  - Correlation holds regardless of distance!
```

---

## Quantum Computing Model {#quantum-model}

### The Quantum Circuit Model

Quantum computation proceeds as:
1. Initialize qubits to |0⟩
2. Apply quantum gates (unitary operations)
3. Measure qubits to get classical output

```
|0⟩ ─────[H]─────[CNOT]─────[M]───→ classical bit
                   │
|0⟩ ──────────────[●]──────[M]───→ classical bit
```

### Key Properties

**Unitary Evolution:**
- All quantum operations (gates) are unitary matrices
- Unitary: U†U = UU† = I (preserves probability)
- Reversible: Can always undo an operation

**Measurement:**
- Collapses superposition to a classical state
- Irreversible (destroys quantum information)
- Probabilities determined by amplitudes

**No-Cloning Theorem:**
- Cannot copy an unknown quantum state
- No "quantum Xerox machine"
- This is why quantum cryptography can detect eavesdropping

---

## Quantum Gates {#quantum-gates}

### Single-Qubit Gates

**Pauli-X (NOT gate):**
```
X = [0 1]    X|0⟩ = |1⟩
    [1 0]    X|1⟩ = |0⟩
```

**Pauli-Z (Phase flip):**
```
Z = [1  0]   Z|0⟩ = |0⟩
    [0 -1]   Z|1⟩ = -|1⟩
```

**Hadamard (Creates superposition):**
```
H = 1/√2 [1  1]   H|0⟩ = |+⟩ = (|0⟩ + |1⟩)/√2
         [1 -1]   H|1⟩ = |−⟩ = (|0⟩ − |1⟩)/√2
```

### Two-Qubit Gates

**CNOT (Controlled-NOT):**
```
CNOT = [1 0 0 0]
       [0 1 0 0]
       [0 0 0 1]
       [0 0 1 0]

CNOT|00⟩ = |00⟩   (control=0, no flip)
CNOT|01⟩ = |01⟩
CNOT|10⟩ = |11⟩   (control=1, flip target)
CNOT|11⟩ = |10⟩
```

### Creating Entanglement

```
|0⟩ ─[H]─[●]─ = |Φ⁺⟩ = (|00⟩ + |11⟩)/√2
|0⟩ ────[⊕]─

Step by step:
  |00⟩ → H⊗I → (|0⟩ + |1⟩)|0⟩/√2 = (|00⟩ + |10⟩)/√2
       → CNOT → (|00⟩ + |11⟩)/√2
```

---

## Shor's Algorithm {#shors-algorithm}

### The Problem

**Integer Factorization:**
```
Given: N = p × q (product of two large primes)
Find: p and q

Example:
  N = 15 = 3 × 5
  N = 21 = 3 × 7
  N = RSA-2048 = ??? (classically infeasible)
```

### Why Factoring Matters for Cryptography

RSA security: Given N = pq, computing p,q is hard.

```
RSA-2048:
  N has 2048 bits (~617 decimal digits)
  Best classical algorithm: General Number Field Sieve
  Time: O(exp(c × n^(1/3) × (log n)^(2/3)))
  Estimated time: Millions of years with all computers on Earth
```

### Shor's Algorithm Overview

1. **Reduce factoring to period-finding**
2. **Use quantum parallelism for period-finding**
3. **Use classical post-processing to extract factors**

### Step 1: Factoring → Period Finding

**Claim:** If we can find the period of f(x) = aˣ mod N, we can factor N.

```
Choose random a coprime to N.
Find smallest r such that: a^r ≡ 1 (mod N)

If r is even:
  a^r - 1 ≡ 0 (mod N)
  (a^(r/2) - 1)(a^(r/2) + 1) ≡ 0 (mod N)

  Let x = a^(r/2) mod N
  Then (x-1)(x+1) ≡ 0 (mod N)

  gcd(x-1, N) or gcd(x+1, N) likely gives a factor!
```

### Step 2: Quantum Period Finding

**Key insight:** Quantum computers can find periods efficiently using the Quantum Fourier Transform.

```
Classical period finding:
  Compute a^0, a^1, a^2, ... mod N until repeat
  Time: O(√N) at best

Quantum period finding:
  Create superposition of all x values
  Evaluate f(x) = a^x mod N in superposition
  Apply QFT to extract period
  Time: O((log N)³) = polynomial!
```

### The Quantum Circuit

```
|0⟩^⊗n ──[H^⊗n]──[U_a]──[QFT†]──[Measure]──→ estimate of r

Where:
  n ≈ 2 log₂ N qubits
  H^⊗n creates superposition of all x
  U_a computes a^x mod N
  QFT† (inverse QFT) extracts period information
```

### Why It Works

After applying H^⊗n:
```
|ψ₁⟩ = 1/√(2^n) Σₓ |x⟩|0⟩
```

After computing f(x) = aˣ mod N:
```
|ψ₂⟩ = 1/√(2^n) Σₓ |x⟩|a^x mod N⟩
```

The second register now contains periodic values!

After QFT on first register:
```
Peaks appear at multiples of 2^n/r
Measurement gives k × (2^n/r) for random k
Continued fractions extract r from multiple measurements
```

### Complexity

```
Shor's Algorithm:
  Qubits needed: O(log N)
  Gates: O((log N)³)
  Time: O((log N)³)

For RSA-2048 (N ≈ 2^2048):
  Qubits: ~4000 logical qubits
  With error correction: ~millions of physical qubits
```

### ECDSA Version

Shor's algorithm also solves the discrete log problem:

```
Given: P (point), Q = kP (another point)
Find: k

Same approach:
  1. Reduce to period finding
  2. Quantum period finding
  3. Extract k

Result: ECDSA broken in polynomial time
```

---

## Grover's Algorithm {#grovers-algorithm}

### The Problem

**Unstructured Search:**
```
Given: Black-box function f: {0,1}^n → {0,1}
       where f(x) = 1 for exactly one x*
Find:  x*
```

### Classical Lower Bound

```
Must query f at least Ω(N) times on average
(where N = 2^n possible inputs)

No structure to exploit - just guessing.
```

### Grover's Improvement

```
Grover's algorithm finds x* in O(√N) queries.

For N = 2^256:
  Classical: 2^256 queries
  Grover:    2^128 queries

Quadratic speedup (not exponential like Shor's)
```

### How It Works

**The Oracle:**
```
O_f |x⟩ = (-1)^f(x) |x⟩

Flips phase of the target state:
  O_f |x*⟩ = -|x*⟩
  O_f |x⟩  = |x⟩  for x ≠ x*
```

**The Diffusion Operator:**
```
D = 2|ψ⟩⟨ψ| - I

Where |ψ⟩ = H^⊗n|0⟩ = uniform superposition

Reflects amplitudes about the mean.
```

**The Algorithm:**
```
1. Start: |ψ⟩ = H^⊗n|0⟩ (uniform superposition)
2. Repeat O(√N) times:
   a. Apply oracle O_f (flip target phase)
   b. Apply diffusion D (amplify target)
3. Measure

Each iteration increases |x*⟩ amplitude by ~2/√N
After √N iterations, probability of measuring x* ≈ 1
```

### Geometric Interpretation

```
State space has two basis vectors:
  |x*⟩ = target state
  |x*⊥⟩ = superposition of all other states

Initial state: |ψ⟩ = sin(θ)|x*⟩ + cos(θ)|x*⊥⟩
  where sin(θ) ≈ 1/√N (small angle)

Each Grover iteration rotates by 2θ toward |x*⟩

After π/(4θ) ≈ √N iterations: state ≈ |x*⟩
```

---

## Impact on Cryptography {#impact}

### Summary Table

| Primitive | Best Classical | Best Quantum | Impact |
|-----------|----------------|--------------|--------|
| RSA-2048 | 2^112 | Poly(n) | **Broken** |
| ECDSA-256 | 2^128 | Poly(n) | **Broken** |
| AES-128 | 2^128 | 2^64 | Weakened |
| AES-256 | 2^256 | 2^128 | Secure |
| SHA-256 preimage | 2^256 | 2^128 | Secure |
| SHA-256 collision | 2^128 | 2^85 | Weakened |

### Detailed Analysis

**Asymmetric Cryptography (RSA, ECDSA, DH):**
```
Security assumption: Factoring/discrete log is hard
Shor's algorithm: Solves in polynomial time
Conclusion: COMPLETELY BROKEN
```

**Symmetric Cryptography (AES, ChaCha20):**
```
Security assumption: No structure to exploit
Grover's algorithm: √N speedup for key search
Conclusion: Double key sizes (AES-256 gives 128-bit post-quantum)
```

**Hash Functions:**
```
Preimage resistance:
  Classical: O(2^n)
  Grover:    O(2^(n/2))
  Conclusion: SHA-256 gives 128-bit post-quantum preimage security

Collision resistance:
  Classical: O(2^(n/2)) (birthday attack)
  Quantum:   O(2^(n/3)) (BHT algorithm)
  Conclusion: SHA-256 gives ~85-bit post-quantum collision security
  Recommendation: Use SHA-384 for collision-critical applications
```

---

## What Survives Quantum Attacks {#survivors}

### Hash-Based Signatures (WOTS+, SPHINCS+)

```
Security basis: Hash preimage resistance

Attack complexity:
  Classical: O(2^n)
  Quantum:   O(2^(n/2))

For n = 256:
  Post-quantum security: 128 bits ✓

Why survives: No structure for Shor to exploit
```

### Lattice-Based Cryptography (Kyber, Dilithium)

```
Security basis: Shortest Vector Problem (SVP), Learning With Errors (LWE)

Best known quantum algorithm: Same as classical
Post-quantum security: Yes (no known quantum speedup for hard lattice problems)

NIST standardized: CRYSTALS-Kyber, CRYSTALS-Dilithium
```

### Code-Based Cryptography (McEliece)

```
Security basis: Decoding random linear codes

History: Proposed in 1978, never broken
Quantum status: No known significant speedup
Downside: Very large public keys (~1 MB)
```

### Symmetric Cryptography with Doubled Keys

```
AES-256:
  Key space: 2^256
  Grover search: 2^128 operations
  Post-quantum security: 128 bits ✓

Recommendation: Use AES-256 instead of AES-128
```

---

## Current State of Quantum Computers {#current-state}

### Progress (as of 2026)

| Company | Qubits | Type | Notes |
|---------|--------|------|-------|
| IBM | 1,000+ | Superconducting | Noisy |
| Google | 100+ | Superconducting | "Quantum supremacy" demo |
| IonQ | 32 | Trapped ion | High fidelity |
| Quantinuum | 56 | Trapped ion | Low error rates |

### Error Correction Challenge

```
Physical qubits: What we have (noisy, error-prone)
Logical qubits: What we need (error-corrected)

Overhead: ~1,000-10,000 physical qubits per logical qubit

To run Shor's algorithm on RSA-2048:
  Logical qubits needed: ~4,000
  Physical qubits needed: ~4,000,000 to 40,000,000

Current state: ~1,000 noisy physical qubits
Gap: ~4 orders of magnitude
```

### Timeline Estimates

```
2024-2026: Noisy Intermediate-Scale Quantum (NISQ) era
           Useful for some problems, not cryptography

2030-2035: Potential early fault-tolerant quantum computers
           Might break weak/small-key cryptography

2035-2040: Cryptographically relevant quantum computers
           RSA-2048, ECDSA-256 at risk

Uncertainty: High (could be faster or slower)
```

### Recommendation

```
Start migration NOW:
  1. Inventory cryptographic usage
  2. Prioritize by data sensitivity and lifetime
  3. Deploy post-quantum algorithms
  4. Use hybrid approaches during transition

"Harvest now, decrypt later" means:
  Data encrypted today could be decrypted in 2035
  If data must stay secret for 10+ years, it's already at risk
```

---

## Key Takeaways

1. **Shor's algorithm** provides exponential speedup for factoring/discrete log → breaks RSA/ECDSA

2. **Grover's algorithm** provides quadratic speedup for search → weakens symmetric crypto, but survives with larger keys

3. **Hash-based signatures** (WOTS+) survive because there's no structure for Shor to exploit

4. **Symmetric crypto** survives with doubled key sizes (AES-256)

5. **Hash functions** mostly survive (SHA-256 for signatures, SHA-384 for collision-critical)

6. **Timeline** is uncertain, but "harvest now, decrypt later" makes migration urgent

7. **Migration** should start now, not when quantum computers arrive

---

## Further Reading

- Shor, P. "Algorithms for Quantum Computation" (1994)
- Grover, L. "A Fast Quantum Mechanical Algorithm for Database Search" (1996)
- Nielsen & Chuang, "Quantum Computation and Quantum Information"
- NIST Post-Quantum Cryptography Project
