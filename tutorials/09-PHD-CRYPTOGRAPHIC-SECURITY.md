# PhD Tutorial: Formal Cryptographic Security

**Time:** 4-6 hours
**Prerequisites:** Graduate-level mathematics, complexity theory, probability theory
**Goal:** Understand provable security frameworks and formal security proofs for hash-based signatures

---

## Table of Contents

1. [Foundations of Provable Security](#foundations)
2. [Security Models and Definitions](#security-models)
3. [Random Oracle Model](#random-oracle)
4. [Hash Function Security Properties](#hash-security)
5. [WOTS+ Security Proof](#wots-proof)
6. [Tightness and Reduction Quality](#tightness)
7. [Post-Quantum Security Models](#post-quantum)
8. [Open Problems](#open-problems)

---

## Foundations of Provable Security {#foundations}

### The Paradigm

Modern cryptography follows the **provable security** paradigm:

```
1. Define a precise security GOAL (what does "secure" mean?)
2. State ASSUMPTIONS (what computational problems are hard?)
3. PROVE: If assumptions hold, construction achieves goal
4. ANALYZE: How tight is the reduction?
```

### Computational vs Information-Theoretic Security

**Information-Theoretic (Unconditional):**
```
Security holds against computationally unbounded adversaries.
Example: One-time pad

Pr[M | C] = Pr[M]  (ciphertext reveals nothing about message)

Requires: |key| ≥ |message|
```

**Computational:**
```
Security holds against polynomial-time adversaries.
Example: AES, RSA, WOTS+

For all PPT adversaries A:
  Advantage(A) ≤ negl(λ)

Where negl(λ) is negligible in security parameter λ
```

### Negligible Functions

A function ε: ℕ → ℝ is **negligible** if:

```
∀ polynomial p(·), ∃ N such that ∀ n > N:
  ε(n) < 1/p(n)

Intuition: Decreases faster than any inverse polynomial.

Examples:
  2^(-n)     - negligible
  2^(-√n)   - negligible
  1/n²       - NOT negligible (polynomial)
  1/n^100    - NOT negligible (polynomial)
```

### Asymptotic vs Concrete Security

**Asymptotic:**
```
Secure if Advantage(A) ≤ negl(λ)

Good for: Theoretical analysis, comparing schemes
Bad for: Real-world parameter selection
```

**Concrete:**
```
Secure if Advantage(A) ≤ ε for specific running time t

Example: AES-128 is (2^128, 2^-128)-secure
  - Adversary with time 2^128 has advantage at most 2^-128

Good for: Setting actual parameters
Bad for: Can be complex, implementation-dependent
```

---

## Security Models and Definitions {#security-models}

### Digital Signature Syntax

A digital signature scheme Σ = (KeyGen, Sign, Verify):

```
KeyGen(1^λ) → (pk, sk)
  Input: Security parameter λ (in unary)
  Output: Public key pk, secret key sk

Sign(sk, m) → σ
  Input: Secret key sk, message m
  Output: Signature σ

Verify(pk, m, σ) → {0, 1}
  Input: Public key pk, message m, signature σ
  Output: 1 (accept) or 0 (reject)

Correctness: For all (pk, sk) ← KeyGen(1^λ), all m:
  Verify(pk, m, Sign(sk, m)) = 1
```

### EUF-CMA: Existential Unforgeability under Chosen Message Attack

The standard security notion for signatures:

```
Experiment EUF-CMA_Σ(A):
  1. (pk, sk) ← KeyGen(1^λ)
  2. (m*, σ*) ← A^{Sign(sk,·)}(pk)
     // A gets pk and oracle access to Sign
     // A can request signatures on chosen messages
  3. Let Q be the set of messages A queried to Sign
  4. Return 1 if:
     - Verify(pk, m*, σ*) = 1
     - m* ∉ Q

Advantage:
  Adv^{EUF-CMA}_Σ(A) = Pr[EUF-CMA_Σ(A) = 1]

Scheme is EUF-CMA secure if for all PPT A:
  Adv^{EUF-CMA}_Σ(A) ≤ negl(λ)
```

### Hierarchy of Security Notions

```
Strongest                                    Weakest
    ↓                                           ↓
SUF-CMA → EUF-CMA → sEUF-CMA → UUF-CMA → EUF-KOA
    ↓                                           ↓
 (Strong)  (Existential)        (Universal)  (Key-Only)

SUF-CMA: Strong UF - can't forge even on queried messages
         with different signature
EUF-CMA: Can't forge signature on NEW message
sEUF-CMA: Selective - choose target before seeing pk
UUF-CMA: Universal - adversary chooses message, we prove
         can't forge on that specific message
EUF-KOA: Key-only attack - no signing queries
```

### One-Time Signatures: EU-CMA (q=1)

For one-time signatures like WOTS+:

```
Experiment EU-CMA_Σ^{q=1}(A):
  1. (pk, sk) ← KeyGen(1^λ)
  2. (m*, σ*) ← A^{Sign(sk,·)}(pk)
     // A may query Sign at most ONCE
  3. Let m_q be the (unique) queried message (if any)
  4. Return 1 if:
     - Verify(pk, m*, σ*) = 1
     - m* ≠ m_q (or no query was made)

This is the correct security model for WOTS+.
```

---

## Random Oracle Model {#random-oracle}

### Definition

A **random oracle** H: {0,1}* → {0,1}^n is:
- A truly random function
- Consistent: H(x) always returns same output for same x
- All parties have oracle access to H

```
Formal model:
  H is chosen uniformly at random from all functions {0,1}* → {0,1}^n

  For each new query x:
    If x was queried before: return stored H(x)
    Else: sample y ← {0,1}^n, store H(x) := y, return y
```

### ROM Security Proofs

**Structure of ROM proofs:**
```
Theorem: Scheme S is secure in the ROM.

Proof:
  Assume adversary A breaks S with advantage ε.
  Construct algorithm B that uses A to break assumption X.

  B simulates random oracle H for A:
    - B answers A's H-queries
    - B can "see" all queries (programming capability)
    - B can make H output specific values (programming)

  B extracts information from A's queries to break X.
```

### Controversy: ROM vs Standard Model

**ROM Criticisms:**
```
1. Random oracles don't exist - hash functions have structure
2. "Uninstantiability" results: Some ROM-secure schemes
   become insecure when H is ANY hash function
3. ROM proofs don't transfer to real world

Counterargument:
  - ROM captures "hash function should look random"
  - No practical attacks exploiting ROM gap
  - Standard model proofs often require strong assumptions
```

**Standard Model Alternatives:**
```
- Programmable hash functions
- Correlation intractability
- Extractable functions

Trade-off: Standard model proofs exist but are:
  - More complex
  - Require stronger/newer assumptions
  - Often less tight
```

---

## Hash Function Security Properties {#hash-security}

### Second Preimage Resistance (SPR)

```
Experiment SPR_H(A):
  1. x ← {0,1}^*        // Random first preimage
  2. x' ← A(x)          // Adversary finds second preimage
  3. Return 1 if x' ≠ x and H(x') = H(x)

Advantage:
  Adv^{SPR}_H(A) = Pr[SPR_H(A) = 1]
```

### Preimage Resistance (OW - One-Wayness)

```
Experiment OW_H(A):
  1. x ← {0,1}^n
  2. y := H(x)
  3. x' ← A(y)
  4. Return 1 if H(x') = y

Advantage:
  Adv^{OW}_H(A) = Pr[OW_H(A) = 1]
```

### Collision Resistance (CR)

```
Experiment CR_H(A):
  1. (x, x') ← A()      // No input, just find collision
  2. Return 1 if x ≠ x' and H(x) = H(x')

Advantage:
  Adv^{CR}_H(A) = Pr[CR_H(A) = 1]
```

### Relationships

```
      CR
     ↙  ↘
   SPR    TCR
     ↘  ↙
      OW

CR ⟹ SPR: Collision finder can find second preimages
CR ⟹ TCR: Target collision resistance (keyed)
SPR ⟹ OW: Second preimage finder can find preimages (weaker)
```

### Quantum Security of Hash Functions

```
Classical:
  OW: O(2^n) queries to find preimage
  CR: O(2^{n/2}) queries (birthday attack)

Quantum (Grover's algorithm):
  OW: O(2^{n/2}) queries

Quantum (BHT algorithm):
  CR: O(2^{n/3}) queries

For n = 256:
  Post-quantum OW security: 128 bits
  Post-quantum CR security: ~85 bits (use larger hash)
```

### Multi-Target Attacks

For WOTS+ with l chains:

```
Single-target preimage: Find x such that H(x) = y

Multi-target preimage: Find x such that H(x) ∈ {y_1, ..., y_l}

Advantage: Factor of l improvement
  Adv^{MT-OW}(A) ≈ l · Adv^{OW}(A)

WOTS+ implication:
  Need n-bit hash for (n - log l)-bit security
  With l = 67, lose ~6 bits
  256-bit hash gives ~122-bit security (post-quantum: ~61 bits)
  Use 512-bit hash for 128-bit post-quantum security
```

---

## WOTS+ Security Proof {#wots-proof}

### WOTS+ Scheme Recap

```
Parameters: n (hash output length), w (Winternitz parameter), l (chain count)
  l₁ = ⌈n/log₂(w)⌉       // Message digits
  l₂ = ⌊log₂(l₁(w-1))/log₂(w)⌋ + 1   // Checksum digits
  l = l₁ + l₂             // Total chains

KeyGen:
  sk = (sk[0], ..., sk[l-1]) ← random n-bit strings
  pk[i] = H^{w-1}(sk[i]) for all i
  pk = (pk[0], ..., pk[l-1])

Sign(sk, M):
  (m[0], ..., m[l-1]) = msg_to_digits(M)  // Includes checksum
  σ[i] = H^{m[i]}(sk[i]) for all i
  σ = (σ[0], ..., σ[l-1])

Verify(pk, M, σ):
  (m[0], ..., m[l-1]) = msg_to_digits(M)
  For all i: check H^{w-1-m[i]}(σ[i]) = pk[i]
```

### Security Theorem

**Theorem (WOTS+ Security):**
```
If H is a second-preimage resistant hash function family,
then WOTS+ is EU-CMA secure for one signing query.

Specifically, for any adversary A making at most 1 signing query
and running in time t:

  Adv^{EU-CMA}_{WOTS+}(A) ≤ l · (w-1) · Adv^{SPR}_H(B)

where B runs in time t' ≈ t + O(l·w) hash evaluations.
```

### Proof Sketch

**Setup:**
```
We construct a reduction B that:
  - Receives SPR challenge (x, H(x))
  - Must find x' ≠ x with H(x') = H(x)
  - Uses WOTS+ forger A to accomplish this
```

**Key Generation Simulation:**
```
B embeds the challenge in one chain:
  - Pick random (i*, j*) ∈ {0,...,l-1} × {0,...,w-2}
  - Generate pk normally, except:
    pk[i*] = H^{w-1-j*}(x)  // Challenge embedded at position j*
```

**Signing Query:**
```
When A queries Sign on message M:
  Compute (m[0], ..., m[l-1]) = msg_to_digits(M)

  If m[i*] > j*:
    // Can't sign - would need to invert past challenge point
    // This is a "bad event" - abort

  Otherwise:
    // Can compute σ[i*] = H^{m[i*]}(sk[i*]) using x
    // σ[i*] = H^{j*-m[i*]}(x) since sk[i*] "is" x at position j*
```

**Extraction:**
```
When A outputs forgery (M*, σ*):
  Compute (m*[0], ..., m*[l-1]) = msg_to_digits(M*)

  If m*[i*] > m[i*] (where m was the signed message):
    // A computed further along chain i* than was revealed
    // This means A found a preimage!

    Let σ*[i*] be A's signature on chain i*
    Let σ[i*] be the signature B gave A (if queried)

    // A computed: H^{m*[i*]}(sk[i*])
    // But only saw: H^{m[i*]}(sk[i*])

    // So A must have computed H^{-1} somewhere
    // Specifically: H(σ'[i*]) = σ*[i*] where σ'[i*] is one step back

    Extract: x' such that H(x') = H(x) for the challenge
```

**Probability Analysis:**
```
Pr[B succeeds] ≥ Pr[A forges] · Pr[good (i*, j*)]

The checksum ensures: For any M* ≠ M, exists i with m*[i] > m[i]
(Can't increase all digits without decreasing checksum)

Pr[correct i*] ≥ 1/l
Pr[correct j*] ≥ 1/(w-1)

Therefore:
  Adv^{SPR}_H(B) ≥ Adv^{EU-CMA}_{WOTS+}(A) / (l·(w-1))

Rearranging:
  Adv^{EU-CMA}_{WOTS+}(A) ≤ l·(w-1)·Adv^{SPR}_H(B)
```

### The Checksum Lemma

**Lemma:** For any two messages M ≠ M' with digit representations (m₀,...,m_{l-1}) and (m'₀,...,m'_{l-1}), there exists some index i where m'ᵢ > mᵢ.

**Proof:**
```
Let C = Σᵢ (w-1-mᵢ) be the checksum for M
Let C' = Σᵢ (w-1-m'ᵢ) be the checksum for M'

Case 1: M and M' differ only in message digits (first l₁)
  If all m'ᵢ ≤ mᵢ with at least one strict, then C' > C
  But checksums are encoded the same way
  So some checksum digit must satisfy m'ᵢ > mᵢ ✓

Case 2: M and M' are identical in message digits
  Then their checksums are identical
  But M ≠ M', so representations must differ somewhere
  Contradiction ✓

Therefore: Always exists i where m'ᵢ > mᵢ
```

---

## Tightness and Reduction Quality {#tightness}

### What is Tightness?

A reduction is **tight** if:
```
Adv^{scheme}(A) ≈ Adv^{assumption}(B)

Loose reduction:
  Adv^{scheme}(A) ≤ L · Adv^{assumption}(B)

where L is the "looseness factor"
```

### Why Tightness Matters

```
Suppose hash has 256-bit security: Adv^{SPR}_H ≤ 2^{-256}

Tight reduction (L = 1):
  Adv^{WOTS+} ≤ 2^{-256}
  Security: 256 bits

Loose reduction (L = l·(w-1) ≈ 67·15 ≈ 2^{10}):
  Adv^{WOTS+} ≤ 2^{10} · 2^{-256} = 2^{-246}
  Security: 246 bits

Lost 10 bits of security due to loose reduction!
```

### WOTS+ Tightness Analysis

```
WOTS+ looseness factor: l · (w-1)

For n=256, w=16:
  l₁ = 64, l₂ = 3, l = 67
  Looseness = 67 · 15 ≈ 1000 ≈ 2^{10}

Security loss: ~10 bits

To achieve 128-bit post-quantum security:
  Need hash with (128 + 10) · 2 = 276-bit security
  Use SHA-384 or SHAKE256 with 384-bit output
```

### Tight Constructions

Some signature schemes have tight reductions:

```
Schnorr signatures (in ROM): Tight to DL
BLS signatures: Tight to CDH
Dilithium: Tight to Module-LWE

WOTS+ and SPHINCS+: Inherently loose
  - Must guess which chain/tree adversary attacks
  - Looseness = number of chains/leaves
```

---

## Post-Quantum Security Models {#post-quantum}

### Quantum Random Oracle Model (QROM)

```
In QROM, adversary can query hash function in superposition:

Classical ROM query: A sends x, receives H(x)

QROM query: A sends |ψ⟩ = Σₓ αₓ|x⟩
            Receives: Σₓ αₓ|x⟩|H(x)⟩

This models quantum adversary accessing hash function.
```

### Challenges in QROM

```
Classical techniques often fail:

1. "Observing queries" - Measuring quantum state disturbs it
2. "Lazy sampling" - Can't sample H(x) only when queried
3. "Rewinding" - Quantum states can't be copied (no-cloning)

New techniques needed:
- Compressed oracles (Zhandry)
- Measure-and-reprogram (Don, Fehr, Majenz, Schaffner)
- Extractable oracles
```

### WOTS+ in QROM

**Theorem (WOTS+ QROM Security):**
```
WOTS+ is EU-CMA secure in the QROM, assuming H is:
  - Quantum one-way
  - Quantum second-preimage resistant

The reduction has additional looseness factor of O(q²)
where q is the number of quantum hash queries.
```

**Proof idea:**
```
Use "compressed oracle" technique:
  - Represent quantum queries as database
  - Extract preimage from database when forgery occurs
  - Measure without disturbing adversary too much
```

### Post-Quantum Concrete Security

```
For 128-bit post-quantum security:

Hash function requirements:
  - Preimage resistance: 256 bits (Grover halves)
  - Collision resistance: 256 bits (BHT gives 2^{n/3})

WOTS+ parameters for 128-bit PQ security:
  - n = 256 (hash output)
  - w = 16
  - l = 67
  - Security: ~122 bits (considering multi-target)

For conservative 128-bit PQ:
  - Use n = 512 or increase w
```

---

## Open Problems {#open-problems}

### Theoretical Questions

**1. Tighter WOTS+ Reductions**
```
Current looseness: l · (w-1)
Question: Can we prove security with tighter reduction?

Partial result: Multi-function variant has tighter security
Trade-off: Requires multiple hash functions
```

**2. Standard Model WOTS+ Security**
```
Current proofs require random oracle model.
Question: Can we prove WOTS+ secure in standard model?

Challenge: Need to instantiate hash chain without ROM
Approach: Use correlation-intractable hash families
```

**3. Quantum Query Complexity**
```
Best known quantum attack: O(2^{n/2}) queries (Grover)
Question: Is this optimal?

Lower bound proofs needed for:
  - Preimage finding
  - Second preimage finding
  - Multi-target variants
```

### Practical Questions

**4. Optimal Parameter Selection**
```
Trade-off: Signature size vs security level vs computation

Question: What are provably optimal parameters for:
  - 128-bit classical security
  - 128-bit post-quantum security
  - Minimal signature size
```

**5. Fault Attack Resistance**
```
Question: Is WOTS+ secure against implementation attacks?

Concerns:
  - Side-channel leakage during signing
  - Fault injection during hash computation
  - Cache timing attacks
```

**6. Hybrid Schemes**
```
Question: How to combine WOTS+ with classical signatures?

Goals:
  - Security if either is secure
  - Minimal size overhead
  - Efficient verification
```

---

## Key Takeaways

1. **Provable security** provides mathematical confidence in cryptographic constructions

2. **WOTS+ security reduces to hash function security** - specifically second-preimage resistance

3. **The checksum is crucial** - ensures forger must invert at least one hash

4. **Reduction is not tight** - lose log₂(l·(w-1)) ≈ 10 bits of security

5. **Post-quantum security** requires QROM analysis and larger parameters

6. **Open problems remain** in tight reductions, standard model proofs, and quantum lower bounds

---

## Further Reading

- Hülsing, A. "W-OTS+ – Shorter Signatures for Hash-Based Signature Schemes" (2013)
- Bernstein, D.J. et al. "SPHINCS: practical stateless hash-based signatures" (2015)
- Zhandry, M. "How to Record Quantum Queries, and Applications to Quantum Indifferentiability" (2019)
- Boneh, D. & Shoup, V. "A Graduate Course in Applied Cryptography" (Chapter on signatures)
- Katz, J. & Lindell, Y. "Introduction to Modern Cryptography" (3rd edition)
