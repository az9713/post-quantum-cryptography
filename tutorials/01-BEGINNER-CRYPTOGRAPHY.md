# Beginner Tutorial: Introduction to Cryptography

**Time:** 30-45 minutes
**Prerequisites:** None
**Goal:** Understand what cryptography is and why it matters for digital security

---

## Table of Contents

1. [What is Cryptography?](#what-is-cryptography)
2. [A Brief History](#brief-history)
3. [The Two Main Problems](#two-problems)
4. [Symmetric Encryption](#symmetric-encryption)
5. [Asymmetric Encryption](#asymmetric-encryption)
6. [Hash Functions](#hash-functions)
7. [Digital Signatures](#digital-signatures)
8. [Why This Matters for Blockchain](#blockchain-connection)
9. [Key Takeaways](#key-takeaways)
10. [Glossary](#glossary)

---

## What is Cryptography? {#what-is-cryptography}

**Cryptography** is the science of secure communication. The word comes from Greek:
- *kryptos* = hidden
- *graphein* = to write

In simple terms, cryptography lets you:
1. **Send secret messages** that only the intended recipient can read
2. **Prove your identity** without revealing your password
3. **Verify data hasn't been tampered with**

### Everyday Examples

You use cryptography every day without realizing it:

| Activity | Cryptography Used |
|----------|-------------------|
| Online banking | HTTPS encryption, digital certificates |
| WhatsApp messages | End-to-end encryption |
| Password storage | Hash functions |
| Logging into websites | Digital signatures, session tokens |
| Bitcoin transactions | ECDSA signatures, SHA-256 hashes |

---

## A Brief History {#brief-history}

### Ancient Cryptography

**Caesar Cipher (50 BC)**

Julius Caesar shifted each letter by 3 positions:

```
Original:  HELLO
Encrypted: KHOOR

A → D, B → E, C → F, ... H → K, E → H, L → O, L → O, O → R
```

This is a **substitution cipher**. The "key" is the shift amount (3).

**Problem:** Easy to break by trying all 26 possible shifts.

### World War II

**Enigma Machine (1930s-1940s)**

The Germans used a mechanical device that performed complex substitutions based on rotating wheels. Breaking Enigma required the work of Alan Turing and others at Bletchley Park.

**Lesson:** Cryptography is an arms race between code-makers and code-breakers.

### Modern Era

**1976: Diffie-Hellman Key Exchange**
First method for two parties to establish a shared secret over an insecure channel.

**1977: RSA**
First practical public-key cryptosystem. Still used today.

**2008: Bitcoin**
First decentralized cryptocurrency, using cryptography for security instead of banks.

---

## The Two Main Problems {#two-problems}

Cryptography solves two fundamental problems:

### Problem 1: Confidentiality

**Question:** How can Alice send a message to Bob that only Bob can read?

```
Alice ----[encrypted message]----> Bob
              ↑
         Eve can see this,
         but can't understand it
```

**Solution:** Encryption

### Problem 2: Authenticity

**Question:** How can Bob verify that a message really came from Alice?

```
Alice ----[message + proof]----> Bob
              ↑
         Eve might try to
         pretend to be Alice
```

**Solution:** Digital signatures

---

## Symmetric Encryption {#symmetric-encryption}

**Symmetric** means both sides use the **same key**.

### How It Works

```
        Same Key                Same Key
           ↓                       ↓
Alice: "Hello" --[Encrypt]--> "Xk9#m" --[Decrypt]--> "Hello" :Bob
                 (scramble)              (unscramble)
```

### Example: AES (Advanced Encryption Standard)

AES is the most widely used symmetric encryption algorithm.

```
Key:        a 256-bit secret number (like a very long password)
Input:      "Attack at dawn"
Output:     "7f4e2b9c1a8d..." (looks like random garbage)
```

Only someone with the same key can decrypt it back to "Attack at dawn".

### The Key Distribution Problem

**Big Problem:** How do Alice and Bob agree on a shared key without Eve intercepting it?

```
Alice wants to send key to Bob
              ↓
Alice ----[key]----> Bob
              ↑
         Eve intercepts
         the key!
```

If Eve gets the key, she can read all messages.

**Solution:** Asymmetric encryption (next section).

### When Symmetric Encryption is Used

- Encrypting files on your computer
- HTTPS (after initial handshake)
- VPN tunnels
- Database encryption

**Advantage:** Very fast (billions of operations per second)
**Disadvantage:** Key distribution problem

---

## Asymmetric Encryption {#asymmetric-encryption}

**Asymmetric** means each person has **two different keys**:
- **Public key:** Can be shared with everyone
- **Private key:** Must be kept secret

### The Magic Property

What the public key encrypts, only the private key can decrypt (and vice versa).

```
Bob's Public Key (known to everyone):  PK_bob
Bob's Private Key (only Bob knows):    SK_bob

Property: Decrypt(SK_bob, Encrypt(PK_bob, message)) = message
```

### How It Solves Key Distribution

```
Step 1: Bob publishes his public key (PK_bob)
        Everyone can see it. That's fine!

Step 2: Alice encrypts message with Bob's public key
        Encrypt(PK_bob, "Hello Bob")

Step 3: Only Bob can decrypt (he has the private key)
        Decrypt(SK_bob, encrypted_message) = "Hello Bob"
```

Eve can see Bob's public key and the encrypted message, but she can't decrypt it because she doesn't have Bob's private key.

### Real-World Analogy

Think of it like a **mailbox**:
- **Public key** = the mailbox slot (anyone can put mail in)
- **Private key** = the key to open the mailbox (only owner has it)

### Common Asymmetric Algorithms

| Algorithm | Based On | Status |
|-----------|----------|--------|
| RSA | Factoring large numbers | Still used, but aging |
| ECDSA | Elliptic curves | Used in Bitcoin/Ethereum |
| Ed25519 | Elliptic curves | Modern, fast |

**Important:** All current asymmetric algorithms will be broken by quantum computers!

---

## Hash Functions {#hash-functions}

A **hash function** takes any input and produces a fixed-size output called a **digest** or **hash**.

### Properties

1. **Deterministic:** Same input always gives same output
2. **Fixed size:** Output is always the same length (e.g., 256 bits)
3. **One-way:** Can't reverse the hash to find the input
4. **Avalanche effect:** Small input change = completely different output

### Example

Using SHA-256 (a common hash function):

```
Input: "Hello"
Hash:  185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969

Input: "Hello!"  (added one character)
Hash:  33b506bd543fd0cf3ef0d00b3e69c42e2fd19e46dc3a8b3c3d3be2e8c0d5b6c7

Completely different!
```

### What Hashes Are Used For

1. **Password storage:** Store hash, not password
2. **File integrity:** Check if file was modified
3. **Blockchain:** Link blocks together
4. **Digital signatures:** Hash message before signing

### Why Hashing is Not Encryption

| Encryption | Hashing |
|------------|---------|
| Reversible (with key) | Not reversible |
| Different output lengths | Fixed output length |
| For confidentiality | For integrity |

### Common Hash Functions

| Name | Output Size | Status |
|------|-------------|--------|
| MD5 | 128 bits | Broken (don't use) |
| SHA-1 | 160 bits | Weak (avoid) |
| SHA-256 | 256 bits | Secure |
| SHA-3 | 256/512 bits | Secure, newer |
| Blake3 | 256 bits | Secure, fast |

---

## Digital Signatures {#digital-signatures}

A **digital signature** proves that a message came from a specific person and hasn't been modified.

### The Concept

Just like a handwritten signature proves you signed a document, a digital signature proves you "signed" digital data.

But digital signatures are actually **better** than handwritten signatures:
- Can't be copied from one document to another
- Any modification to the document invalidates the signature
- Mathematically verifiable

### How It Works

```
Step 1: Alice has a private key (SK_alice) and public key (PK_alice)

Step 2: Alice signs a message
        signature = Sign(SK_alice, "I owe Bob $100")

Step 3: Anyone can verify using Alice's public key
        Verify(PK_alice, "I owe Bob $100", signature) = true
```

### Properties

1. **Only Alice can create signatures** (needs private key)
2. **Anyone can verify signatures** (only needs public key)
3. **Signature is bound to the message** (can't move to another message)

### Why Hash Before Signing?

In practice, we hash the message first, then sign the hash:

```
message → Hash(message) → Sign(hash) → signature
```

Why?
1. Signing is slow; hashing a long document to 256 bits makes signing fast
2. Security: signing raw data can leak information

---

## Why This Matters for Blockchain {#blockchain-connection}

### Bitcoin's Cryptographic Foundation

Bitcoin uses cryptography for everything:

| Feature | Cryptography Used |
|---------|-------------------|
| Wallets | Public/private key pairs |
| Addresses | Hash of public key |
| Transactions | Digital signatures (ECDSA) |
| Mining | Hash puzzles (SHA-256) |
| Block linking | Hash chains |

### How a Bitcoin Transaction Works

```
1. Alice has private key SK_alice
2. Alice's address = Hash(PublicKey(SK_alice))
3. Alice creates transaction: "Send 1 BTC to Bob's address"
4. Alice signs: signature = Sign(SK_alice, transaction)
5. Network verifies: Verify(PK_alice, transaction, signature)
6. If valid, transaction is included in a block
```

### The Quantum Threat

Here's the problem this project addresses:

**ECDSA** (the signature algorithm Bitcoin/Ethereum use) is based on elliptic curve mathematics. Quantum computers running **Shor's Algorithm** can break this.

```
Classical computer: Breaking ECDSA takes mass-of-the-universe years
Quantum computer:   Breaking ECDSA takes hours/days
```

**WOTS+** (what this project implements) uses only hash functions, which quantum computers can't easily break.

---

## Key Takeaways {#key-takeaways}

1. **Cryptography** = science of secure communication

2. **Symmetric encryption** = same key encrypts and decrypts (fast, but key sharing problem)

3. **Asymmetric encryption** = public key encrypts, private key decrypts (solves key sharing)

4. **Hash functions** = one-way functions producing fixed-size output (for integrity)

5. **Digital signatures** = prove authorship and integrity (private key signs, public key verifies)

6. **Blockchain relies on cryptography** for security instead of trusted institutions

7. **Quantum computers threaten** current asymmetric cryptography (RSA, ECDSA)

8. **Post-quantum cryptography** (like WOTS+) uses quantum-resistant techniques

---

## Glossary {#glossary}

| Term | Definition |
|------|------------|
| **Plaintext** | The original, readable message |
| **Ciphertext** | The encrypted, unreadable message |
| **Key** | Secret value used to encrypt/decrypt |
| **Encryption** | Converting plaintext to ciphertext |
| **Decryption** | Converting ciphertext back to plaintext |
| **Hash** | Fixed-size output of a hash function |
| **Digest** | Another word for hash |
| **Public key** | Key that can be shared publicly |
| **Private key** | Key that must be kept secret |
| **Digital signature** | Cryptographic proof of authorship |
| **Algorithm** | Step-by-step procedure for computation |
| **Symmetric** | Using the same key for both operations |
| **Asymmetric** | Using different keys for each operation |

---

## What's Next?

Now that you understand cryptography basics:

- **[02-BEGINNER-BLOCKCHAIN](02-BEGINNER-BLOCKCHAIN.md)** - Learn how blockchain uses cryptography
- **[03-BEGINNER-QUANTUM](03-BEGINNER-QUANTUM.md)** - Understand the quantum computing threat
- **[04-INTERMEDIATE-HASH-FUNCTIONS](04-INTERMEDIATE-HASH-FUNCTIONS.md)** - Deep dive into hash functions

---

## Self-Check Questions

1. What's the difference between symmetric and asymmetric encryption?
2. Why can't you reverse a hash function?
3. How does a digital signature prove authorship?
4. Why do we hash a message before signing it?
5. What cryptographic algorithms does Bitcoin use?

*Answers: Review the relevant sections above. If you can explain these concepts in your own words, you're ready for the next tutorial!*
