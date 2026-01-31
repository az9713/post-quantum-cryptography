# PhD Tutorial: The Post-Quantum Cryptography Landscape

**Time:** 4-6 hours
**Prerequisites:** Graduate cryptography, complexity theory, abstract algebra
**Goal:** Comprehensive understanding of post-quantum cryptographic families and research frontiers

---

## Table of Contents

1. [The Post-Quantum Imperative](#imperative)
2. [NIST Standardization](#nist)
3. [Lattice-Based Cryptography](#lattices)
4. [Hash-Based Signatures](#hash-based)
5. [Code-Based Cryptography](#code-based)
6. [Multivariate Cryptography](#multivariate)
7. [Isogeny-Based Cryptography](#isogenies)
8. [Hybrid Approaches](#hybrid)
9. [Research Frontiers](#frontiers)
10. [Migration Strategies](#migration)

---

## The Post-Quantum Imperative {#imperative}

### Quantum Computing Timeline

```
Historical Milestones:
1994: Shor's algorithm published (polynomial factoring)
1996: Grover's algorithm (quadratic search speedup)
2019: Google "quantum supremacy" claim (53 qubits)
2023: IBM Condor (1,121 qubits, noisy)
2024: Error-corrected logical qubits demonstrated
2026: ~1,000-2,000 physical qubits, improving fidelity

Projections (high uncertainty):
2028-2030: Early fault-tolerant systems
2030-2035: Cryptographically relevant quantum computers (CRQC)
2035-2040: Widespread availability
```

### The "Harvest Now, Decrypt Later" Threat

```
Timeline analysis:

2026: Data encrypted with RSA-2048
      Adversary captures ciphertext (trivial)

2035: CRQC available
      Adversary decrypts captured data
      9 years later, but data may still be sensitive

Implication:
  If data must remain secret for X years,
  and CRQC arrives in Y years,
  then migration must complete by year (Y - X) ago.

For 20-year secrets and 2035 CRQC:
  Migration should have started by 2015!
```

### Quantum Threat to Cryptographic Primitives

```
Algorithm          Classical    Quantum      Status
─────────────────────────────────────────────────────
RSA-2048           ~2^112       Poly(n)      BROKEN
ECDSA-256          ~2^128       Poly(n)      BROKEN
DH-2048            ~2^112       Poly(n)      BROKEN
AES-128            2^128        2^64         WEAKENED
AES-256            2^256        2^128        SECURE
SHA-256 preimage   2^256        2^128        SECURE
SHA-256 collision  2^128        2^85         WEAKENED
WOTS+ (n=256)      2^256        2^128        SECURE
Dilithium-2        ~2^128       ~2^128       SECURE
Kyber-768          ~2^128       ~2^128       SECURE
```

---

## NIST Standardization {#nist}

### The NIST Post-Quantum Cryptography Project

```
Timeline:
2016: Call for proposals issued
2017: 82 submissions received
2019: Round 2 - 26 candidates
2020: Round 3 - 7 finalists + 8 alternates
2022: Initial selections announced
2024: FIPS 203, 204, 205 finalized
2026: Additional standards in development
```

### Finalized Standards (August 2024)

**FIPS 203: ML-KEM (Module Lattice Key Encapsulation Mechanism)**
```
Based on: CRYSTALS-Kyber
Type: Key encapsulation mechanism (KEM)
Assumption: Module Learning With Errors (MLWE)

Parameter sets:
  ML-KEM-512:  ~128-bit security, smallest keys
  ML-KEM-768:  ~192-bit security (recommended)
  ML-KEM-1024: ~256-bit security, largest keys

Performance (reference implementation):
  KeyGen: ~30 μs
  Encaps: ~40 μs
  Decaps: ~50 μs
  Public key: 800-1568 bytes
  Ciphertext: 768-1568 bytes
```

**FIPS 204: ML-DSA (Module Lattice Digital Signature Algorithm)**
```
Based on: CRYSTALS-Dilithium
Type: Digital signature
Assumption: Module Learning With Errors (MLWE)

Parameter sets:
  ML-DSA-44: ~128-bit security
  ML-DSA-65: ~192-bit security (recommended)
  ML-DSA-87: ~256-bit security

Performance:
  KeyGen: ~50 μs
  Sign:   ~100 μs
  Verify: ~80 μs
  Public key: 1312-2592 bytes
  Signature: 2420-4595 bytes
```

**FIPS 205: SLH-DSA (Stateless Hash-Based Digital Signature Algorithm)**
```
Based on: SPHINCS+
Type: Digital signature (hash-based)
Assumption: Hash function security only

Parameter sets (SHA-256 or SHAKE):
  SLH-DSA-128s/f: 128-bit security
  SLH-DSA-192s/f: 192-bit security
  SLH-DSA-256s/f: 256-bit security
  (s = small signature, f = fast signing)

Performance (128f):
  KeyGen: ~2 ms
  Sign:   ~50 ms
  Verify: ~5 ms
  Public key: 32 bytes
  Signature: 17,088 bytes
```

### Ongoing Standardization

```
Round 4 candidates (additional signatures):
  - BIKE (code-based KEM)
  - Classic McEliece (code-based KEM)
  - HQC (code-based KEM)

On-ramp call (2023):
  - Additional signature schemes
  - Different design approaches
  - Focus on efficiency
```

---

## Lattice-Based Cryptography {#lattices}

### Mathematical Foundation

**Lattice Definition:**
```
A lattice L ⊂ ℝⁿ is the set of all integer linear combinations
of linearly independent vectors b₁, ..., bₙ ∈ ℝⁿ:

L = {Σᵢ zᵢbᵢ : zᵢ ∈ ℤ}

The vectors {b₁, ..., bₙ} form a basis for L.

Fundamental domain: Parallelepiped spanned by basis vectors
Determinant: det(L) = |det([b₁ | ... | bₙ])|
```

### Hard Lattice Problems

**Shortest Vector Problem (SVP):**
```
Given: Lattice L with basis B
Find: Shortest non-zero vector v ∈ L

Exact SVP: Find v with ||v|| = λ₁(L)
Approximate γ-SVP: Find v with ||v|| ≤ γ · λ₁(L)

Where λ₁(L) is the length of the shortest vector.

Complexity:
  Exact SVP: NP-hard (for certain norms)
  γ-SVP: Polynomial-time solvable for γ ≥ 2^{n/2}
          Believed hard for γ = poly(n)
```

**Closest Vector Problem (CVP):**
```
Given: Lattice L with basis B, target vector t ∈ ℝⁿ
Find: Vector v ∈ L closest to t

Reduction: CVP is at least as hard as SVP
```

**Learning With Errors (LWE):**
```
Parameters: n (dimension), q (modulus), χ (error distribution)

Problem:
  Given: (A, b = As + e mod q)
         where A ← ℤqⁿˣᵐ, s ← ℤqⁿ, e ← χᵐ
  Find: s (search version)
   or
  Distinguish: b from uniform (decision version)

Hardness: Quantum reduction from worst-case lattice problems
          Decision LWE ≈ SVP on ideal lattices
```

**Ring-LWE and Module-LWE:**
```
Ring-LWE: Work in polynomial ring R = ℤq[x]/(xⁿ+1)
  - More structured, smaller keys
  - Security concerns: lattice structure exploitable?

Module-LWE: Work in Rᵏ (vectors of ring elements)
  - Balance between structure and security
  - Used in Kyber and Dilithium
```

### Kyber/ML-KEM Deep Dive

**Kyber Construction:**
```
Public key: (A, t = As + e) where A ∈ Rqᵏˣᵏ, s,e ∈ Rqᵏ
Secret key: s

Encapsulation:
  1. Sample r, e₁, e₂ ← χ
  2. u = Aᵀr + e₁
  3. v = tᵀr + e₂ + ⌈q/2⌉·m
  4. Ciphertext: (u, v)
  5. Shared secret: H(m)

Decapsulation:
  1. Compute v - sᵀu = ⌈q/2⌉·m + small error
  2. Round to recover m
  3. Verify: re-encapsulate and check
```

**Security Analysis:**
```
IND-CCA2 security from:
  - MLWE hardness
  - Fujisaki-Okamoto transform (CPA → CCA)

Best known attacks:
  - BKZ lattice reduction: Time 2^{Θ(n)}
  - Primal/dual attacks: Similar complexity
  - Quantum algorithms: No significant speedup known

Concrete security (ML-KEM-768):
  Core-SVP: ~180 bits
  With reductions: ~128 bits post-quantum
```

### Dilithium/ML-DSA Deep Dive

**Dilithium Construction (Fiat-Shamir with aborts):**
```
KeyGen:
  Sample A ∈ Rqᵏˣˡ, s₁ ∈ Sηˡ, s₂ ∈ Sηᵏ
  t = As₁ + s₂
  pk = (A, t), sk = (A, t, s₁, s₂)

Sign(sk, M):
  loop:
    y ← Sγ₁ˡ (random masking vector)
    w = Ay
    c = H(M || HighBits(w))  // Challenge
    z = y + c·s₁
    if rejection_condition(z, cs₂): continue
    return σ = (z, c, hint)

Verify(pk, M, σ):
  w' = Az - ct
  c' = H(M || HighBits(w', hint))
  return (c = c') and ||z||∞ < bound
```

**Rejection Sampling:**
```
Why needed:
  z = y + cs₁ leaks information about s₁
  Rejection sampling makes z independent of s₁

Technique:
  Accept z with probability proportional to ρ(z)/ρ(z-cs₁)
  where ρ is the masking distribution

Result: ~4 repetitions on average
```

---

## Hash-Based Signatures {#hash-based}

### Hierarchy of Hash-Based Signatures

```
One-Time Signatures (OTS)
├── Lamport OTS (1979)
├── Winternitz OTS (1979)
└── WOTS+ (2013)
    └── Parameter: Winternitz parameter w

Few-Time Signatures
├── HORS (2002)
└── HORST (used in SPHINCS)

Many-Time Signatures (via Merkle trees)
├── MSS (Merkle Signature Scheme, 1989)
├── XMSS (2011) - Stateful
├── LMS (2019) - NIST SP 800-208
└── SPHINCS+ (2015/2019) - Stateless
```

### SPHINCS+ / SLH-DSA Architecture

```
Three main components:

1. WOTS+ - One-time signatures for each leaf
2. FORS (Forest of Random Subsets) - Few-time signature
3. Hypertree - Tree of trees for address space

Structure:
         [Hypertree Root]
              ↓
    ┌────────┴────────┐
    ↓                 ↓
[XMSS Tree 1]    [XMSS Tree 2]  ...
    │                 │
    ↓                 ↓
  WOTS+             WOTS+        ...
    │                 │
    ↓                 ↓
  FORS              FORS         ...
    │                 │
    ↓                 ↓
 Message            Message

Each level uses WOTS+ to sign root of level below.
FORS at bottom signs message hash.
```

### FORS (Forest of Random Subsets)

```
Parameters: k (number of trees), t (leaves per tree)

Structure: k binary trees, each with t leaves

Signing:
  1. Hash message to k log₂(t)-bit indices
  2. For each index, reveal leaf and authentication path
  3. Include k tree roots

Verification:
  1. Recompute indices from message hash
  2. Verify each authentication path
  3. Check roots match signed roots

Security: Existential forgery requires finding collision
          or preimage for at least one tree
```

### Stateful vs Stateless

**Stateful (XMSS, LMS):**
```
Pros:
  - Smaller signatures (~2.5 KB)
  - Faster signing
  - Tighter security proof

Cons:
  - Must track state (which leaves used)
  - State synchronization across devices
  - Catastrophic failure if state lost/duplicated
```

**Stateless (SPHINCS+):**
```
Pros:
  - No state management
  - Safe for any deployment model
  - Conservative security assumption

Cons:
  - Larger signatures (17-49 KB)
  - Slower signing
  - Looser security reduction
```

### Security Proof Sketch for SPHINCS+

```
Theorem: SPHINCS+ is EUF-CMA secure if:
  - PRF is secure (pseudorandom function)
  - Hash function is SPR (second-preimage resistant)
  - Hash function is TCR (target collision resistant)

Proof structure:
  1. Replace PRF with random function (indistinguishable)
  2. Adversary forges ⟹ one of:
     a. FORS collision/second-preimage
     b. WOTS+ forgery at some tree level
     c. Merkle tree collision

  3. Each leads to hash function break

Looseness factor: O(number of signing queries × hypertree height)
```

---

## Code-Based Cryptography {#code-based}

### Error-Correcting Codes Primer

**Linear Codes:**
```
An [n, k, d] linear code C over 𝔽q is a k-dimensional
subspace of 𝔽qⁿ with minimum distance d.

Generator matrix G ∈ 𝔽qᵏˣⁿ: C = {mG : m ∈ 𝔽qᵏ}
Parity-check matrix H ∈ 𝔽q⁽ⁿ⁻ᵏ⁾ˣⁿ: C = {c : Hcᵀ = 0}

Distance: d = min{weight(c) : c ∈ C, c ≠ 0}
Error correction: Can correct up to ⌊(d-1)/2⌋ errors
```

**Goppa Codes:**
```
Binary Goppa code Γ(L, g):
  L = {α₁, ..., αₙ} ⊂ 𝔽₂ₘ (support)
  g(x) ∈ 𝔽₂ₘ[x] (Goppa polynomial of degree t)

Code: {c ∈ 𝔽₂ⁿ : Σᵢ cᵢ/(x - αᵢ) ≡ 0 mod g(x)}

Properties:
  - Minimum distance ≥ 2t + 1
  - Efficient decoding (Patterson's algorithm)
  - Indistinguishable from random (conjectured)
```

### The McEliece Cryptosystem (1978)

```
KeyGen:
  1. Generate random [n, k, d] Goppa code with generator G'
  2. Sample random k×k invertible matrix S
  3. Sample random n×n permutation matrix P
  4. Compute G = SG'P (disguised generator)
  5. pk = G, sk = (S, G', P)

Encrypt(pk, m):
  c = mG + e where e is random error of weight t

Decrypt(sk, c):
  1. Compute c' = cP⁻¹ = mSG' + eP⁻¹
  2. Decode c' using Goppa decoder → mS
  3. Compute m = (mS)S⁻¹
```

**Security:**
```
Problem: Decoding random linear codes is NP-hard

Best attacks:
  - Information set decoding (ISD)
  - Generalized birthday attacks

Quantum speedup: Only Grover's √ speedup on ISD
  Classical: 2^{0.0597n}
  Quantum:   2^{0.0299n}

For n = 6960, t = 119 (Classic McEliece):
  Classical: ~262 bits
  Quantum:   ~131 bits
```

### Classic McEliece Parameters

```
Parameter Set    n      k      t     pk size   ct size
──────────────────────────────────────────────────────
mceliece348864  3488  2720   64    261 KB    128 B
mceliece460896  4608  3360   96    524 KB    188 B
mceliece6688128 6688  5024   128   1 MB      240 B
mceliece6960119 6960  5413   119   1 MB      226 B
mceliece8192128 8192  6528   128   1.4 MB    240 B

Trade-off: Very large public keys, small ciphertexts
Use case: Long-term key storage, pre-shared keys
```

### BIKE and HQC

**BIKE (Bit Flipping Key Encapsulation):**
```
Based on: QC-MDPC codes (quasi-cyclic moderate density parity-check)
Advantage: Much smaller keys than McEliece
Challenge: Decoding failure rate (mitigated in BIKE-3)

Public key size: ~1.5-3 KB
Ciphertext size: ~1.5-3 KB
```

**HQC (Hamming Quasi-Cyclic):**
```
Based on: Quasi-cyclic codes + syndrome decoding
Advantage: Simple construction, provable reduction
Disadvantage: Larger parameters than BIKE

Public key size: ~3-8 KB
Ciphertext size: ~6-14 KB
```

---

## Multivariate Cryptography {#multivariate}

### Mathematical Foundation

**Multivariate Quadratic (MQ) Problem:**
```
Given: System of m quadratic polynomials in n variables over 𝔽q
       p₁(x₁,...,xₙ) = 0
       ⋮
       pₘ(x₁,...,xₙ) = 0

Find: Solution (x₁,...,xₙ) ∈ 𝔽qⁿ

Complexity: NP-complete for random systems
Quantum: No known significant speedup
```

### Oil and Vinegar Signature Scheme

```
Idea: Separate variables into "oil" (o) and "vinegar" (v)
      Quadratic terms only in vinegar variables

Public key: Quadratic map P: 𝔽qⁿ → 𝔽qᵐ (hides structure)
Secret key: Affine transformations S, T and structured map F

Central map F:
  F(o,v) = Σ αᵢⱼ vᵢvⱼ + Σ βᵢⱼ oᵢvⱼ + Σ γᵢ oᵢ + Σ δᵢ vᵢ + η
           (vinegar)    (cross terms)   (oil)    (vinegar) (const)

  Note: No oil×oil terms! Makes inversion tractable.

Sign:
  1. Hash message to target y ∈ 𝔽qᵐ
  2. Choose random vinegar v
  3. Solve linear system for oil o (linear in o!)
  4. Signature: σ = S⁻¹(o,v)

Verify:
  Check P(σ) = Hash(message)
```

### Rainbow (Broken in 2022)

```
Rainbow: Multi-layer Oil-and-Vinegar

Structure: Variables partitioned into layers V₁ ⊂ V₂ ⊂ ... ⊂ Vₖ
           Each layer adds more "oil" variables

Attack (Beullens, 2022):
  Intersection attack: Find invariant subspace
  Complexity: O(n^4) field operations
  Practical: Break Rainbow-I in 53 hours on laptop

Status: NIST removed from consideration
Lesson: Structure in multivariate schemes is dangerous
```

### Current State of Multivariate Cryptography

```
Broken/weakened:
  - Rainbow (2022 attack)
  - GeMSS (algebraic attacks)

Still standing:
  - UOV (classic Oil-and-Vinegar)
  - MAYO (optimized UOV variant)
  - VOX (smaller keys)

Advantages:
  - Very small signatures (~100-200 bytes)
  - Fast verification
  - No lattice structure

Disadvantages:
  - Large public keys (tens of KB)
  - History of breaks
  - Complex security analysis
```

---

## Isogeny-Based Cryptography {#isogenies}

### Elliptic Curve Isogenies

**Definition:**
```
An isogeny φ: E₁ → E₂ is a surjective group homomorphism
between elliptic curves that is also a morphism of varieties.

Properties:
  - Kernel ker(φ) is a finite subgroup
  - Degree deg(φ) = |ker(φ)|
  - For every finite subgroup G ⊂ E, exists unique isogeny
    with kernel G (Vélu's formulas)
```

**Isogeny Graph:**
```
Vertices: Elliptic curves (up to isomorphism)
Edges: Isogenies of fixed degree ℓ

For supersingular curves over 𝔽p²:
  - Graph is ℓ+1 regular (Ramanujan graph)
  - Expander: Random walks mix quickly
  - Finding paths between vertices = hard problem
```

### SIKE/SIDH (Broken in 2022)

```
Original idea (SIDH):
  - Alice walks from E₀ using 2-isogenies
  - Bob walks from E₀ using 3-isogenies
  - Share auxiliary points to compute shared curve

Attack (Castryck-Decru-Robert, 2022):
  - Auxiliary points leak information
  - Polynomial-time attack using theta functions
  - Complete break of SIKE/SIDH

Impact: Major setback for isogeny cryptography
```

### Post-SIDH Isogeny Cryptography

**CSIDH (Commutative SIDH):**
```
Based on: Class group action on supersingular curves

Structure: Curves E over 𝔽p with End(E) = O_K (maximal order)
Action: [a] * E = E/E[a] (ideal a acts on curve)
Commutativity: [a] * ([b] * E) = [b] * ([a] * E)

Public key: E' = [sk] * E₀
Key exchange: E_shared = [sk_A] * E_B = [sk_B] * E_A

Status: No known polynomial attack, but slower
```

**SQISign (Short Quaternion Isogeny Signature):**
```
Based on: Deuring correspondence (isogenies ↔ quaternion ideals)

Signature size: 177 bytes (smallest PQ signature!)
Public key: 64 bytes
Security: 128 bits

Disadvantage: Very slow signing (~seconds)
Research: Active optimization efforts
```

---

## Hybrid Approaches {#hybrid}

### Rationale for Hybrid Cryptography

```
Uncertainty justifies hedging:
  1. PQ algorithms less cryptanalyzed than classical
  2. Implementation attacks less understood
  3. Parameter selection may be wrong

Hybrid principle: Combine classical + PQ
  Security: Broken only if BOTH are broken
  Overhead: Sum of key/signature sizes
```

### Hybrid Key Exchange

**Concatenated Hybrid:**
```
Classical: ECDH key K₁
PQ:        Kyber key K₂
Combined:  K = KDF(K₁ || K₂)

Security: K is secure if either ECDH or Kyber is secure
(Assuming KDF is secure)
```

**Real-World Deployments:**
```
TLS 1.3 + PQ:
  - X25519Kyber768 (Chrome, CloudFlare)
  - secp384r1+ML-KEM-768

SSH:
  - sntrup761x25519 (OpenSSH 9.0+)

Signal Protocol:
  - X25519 + PQXDH (Kyber1024)
```

### Hybrid Signatures

**Concatenated Signatures:**
```
sig_hybrid = sig_classical || sig_pq

Verify: Accept if BOTH verify

Issue: Signature size is sum of both
```

**Nested/Composite Signatures:**
```
sig = Sign_PQ(Sign_classical(m) || m)

Verify: Check outer PQ signature, then inner classical

Security: Slightly different guarantees
```

### Standards for Hybrid

```
IETF drafts:
  - draft-ietf-tls-hybrid-design
  - draft-ietf-pquip-hybrid-signature-spectrums

X.509 Hybrid Certificates:
  - Catalyst hybrid certificates (contains both keys)
  - Composite certificates (single key algorithm)

Challenge: Backwards compatibility with legacy systems
```

---

## Research Frontiers {#frontiers}

### Efficiency Improvements

**Lattice Optimizations:**
```
- NTT (Number Theoretic Transform): O(n log n) multiplication
- AVX-512 implementations: 10-100× speedups
- Compressed representations: Smaller keys/signatures
```

**Hash-Based Optimizations:**
```
- SPHINCS-α: Faster SPHINCS+ variant
- Gravity-SPHINCS: Hardware-optimized
- WOTS-T: Threshold WOTS signatures
```

### New Primitives

**Functional Encryption:**
```
Post-quantum FE: Decrypt to f(m) not m
  - Lattice-based FE exists
  - Limited functionalities currently
```

**Homomorphic Encryption:**
```
Fully Homomorphic Encryption (FHE):
  - Compute on encrypted data
  - All practical FHE is lattice-based
  - Already post-quantum!

TFHE, BGV, CKKS: Production FHE schemes
```

**Zero-Knowledge Proofs:**
```
Post-quantum ZK:
  - STARKs: Already hash-based (quantum-safe)
  - Lattice-based SNARKs: Active research
  - MPC-in-the-head: Symmetric-key based

Challenge: Efficient lattice-based SNARKs
```

### Theoretical Questions

**Hardness Assumptions:**
```
1. LWE hardness in presence of quantum advice?
2. Optimal parameters for post-quantum security?
3. New quantum algorithms for lattice problems?
4. Tighter security reductions?
```

**New Attack Vectors:**
```
1. Algebraic attacks on structured lattices
2. Side-channel attacks on PQ implementations
3. Fault attacks on deterministic signatures
4. Combined classical+quantum attacks
```

### Standardization Evolution

```
Upcoming:
  - NIST Round 4 (additional signatures)
  - ISO/IEC 14888-4 (hash-based signatures)
  - IETF PQUIP working group

Long-term:
  - Cryptographic agility standards
  - Automated algorithm selection
  - Hardware security module updates
```

---

## Migration Strategies {#migration}

### Assessment Phase

```
1. Inventory cryptographic usage:
   - TLS certificates
   - Code signing
   - Data encryption at rest
   - Key management systems
   - Custom cryptographic code

2. Classify by:
   - Data sensitivity
   - Lifetime requirements
   - External dependencies
   - Regulatory requirements
```

### Prioritization Framework

```
Priority 1 (Immediate):
  - Long-lifetime secrets (>10 years)
  - Government/defense systems
  - Financial infrastructure
  - Healthcare records

Priority 2 (Near-term):
  - Certificate authorities
  - VPNs and remote access
  - Database encryption
  - Email encryption

Priority 3 (Medium-term):
  - Web server TLS
  - IoT devices
  - Consumer applications
```

### Implementation Approaches

**Crypto-Agility:**
```
Design systems to swap algorithms:
  - Abstract cryptographic interfaces
  - Version-tagged encrypted data
  - Graceful degradation paths

Example:
  encrypt(data) →
    if supports_pq():
      return pq_encrypt(data)
    else:
      return classical_encrypt(data)
```

**Hybrid Deployment:**
```
Phase 1: Add PQ alongside classical
Phase 2: Validate PQ performance
Phase 3: Make PQ mandatory
Phase 4: Deprecate classical
```

**Testing Strategy:**
```
1. Algorithm conformance tests
2. Interoperability testing
3. Performance benchmarking
4. Security validation
5. Backwards compatibility
```

### Blockchain-Specific Considerations

```
Unique challenges:
  1. Immutable history (can't retrofit old transactions)
  2. Decentralized consensus (no central authority)
  3. Smart contract compatibility
  4. Address format changes

Approaches:
  1. New address types (SegWit-style upgrade)
  2. Account abstraction (Ethereum's approach)
  3. Signature aggregation (STARKs for WOTS+)
  4. Hybrid signatures during transition
```

---

## Key Takeaways

1. **NIST has standardized** ML-KEM, ML-DSA, and SLH-DSA for post-quantum cryptography

2. **Lattice-based cryptography** offers the best balance of security, performance, and key sizes

3. **Hash-based signatures** provide conservative security but larger sizes (SPHINCS+: 17-49 KB)

4. **Code-based cryptography** is proven over decades but has very large public keys

5. **Multivariate and isogeny** schemes suffered major breaks but research continues

6. **Hybrid approaches** provide security hedging during the transition period

7. **Migration should start now** - "harvest now, decrypt later" makes this urgent

8. **Crypto-agility** is essential for long-term security

---

## Further Reading

### Standards and Reports
- NIST SP 800-208: LMS and XMSS Hash-Based Signatures
- NIST FIPS 203/204/205: ML-KEM, ML-DSA, SLH-DSA
- ETSI TR 103 619: Migration to Post-Quantum Cryptography

### Textbooks
- Peikert, C. "A Decade of Lattice Cryptography"
- Bernstein, D.J. & Lange, T. "Post-Quantum Cryptography"

### Key Papers
- Regev, O. "On Lattices, Learning with Errors, Random Linear Codes..." (2009)
- Hülsing, A. et al. "SPHINCS+" (2019)
- Castryck, W. & Decru, T. "An Efficient Key Recovery Attack on SIDH" (2022)
- Beullens, W. "Breaking Rainbow Takes a Weekend on a Laptop" (2022)

### Implementation Resources
- PQClean: Clean implementations of PQ algorithms
- liboqs: Open Quantum Safe library
- Cloudflare Circl: Go cryptographic library
