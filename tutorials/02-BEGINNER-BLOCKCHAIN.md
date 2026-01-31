# Beginner Tutorial: Blockchain, Bitcoin, and Ethereum

**Time:** 30-45 minutes
**Prerequisites:** Basic understanding of cryptography (Tutorial 01 helpful but not required)
**Goal:** Understand how blockchain works and why cryptographic signatures are essential

---

## Table of Contents

1. [The Problem Blockchain Solves](#the-problem)
2. [What is a Blockchain?](#what-is-blockchain)
3. [Bitcoin Explained](#bitcoin)
4. [Ethereum Explained](#ethereum)
5. [Wallets and Addresses](#wallets)
6. [Transactions in Detail](#transactions)
7. [Mining and Consensus](#mining)
8. [Why Security Matters](#security)
9. [Key Takeaways](#takeaways)
10. [Glossary](#glossary)

---

## The Problem Blockchain Solves {#the-problem}

### The Double-Spending Problem

Imagine you have a digital photo. You can copy it infinitely. Now imagine that photo represents $100. What stops you from "spending" that $100 with multiple people?

```
You have: digital_100_dollars.jpg

You send to Alice: digital_100_dollars.jpg ✓
You send to Bob:   digital_100_dollars.jpg ✓
You send to Carol: digital_100_dollars.jpg ✓

Problem: You just spent the same $100 three times!
```

### The Traditional Solution: Banks

Banks solve this by keeping a central ledger:

```
Bank's Ledger:
  Your account: $100

Transaction: You → Alice $100
  Your account: $0
  Alice account: $100

You try to send $100 to Bob:
  REJECTED - insufficient funds
```

**Problem with banks:**
- You must trust the bank
- Bank can freeze your account
- Bank can be hacked
- Bank charges fees
- Not everyone has bank access

### Satoshi Nakamoto's Insight (2008)

What if everyone kept a copy of the ledger, and we used cryptography to ensure honesty?

```
Instead of:
  Bank keeps THE ledger

We have:
  Everyone keeps A COPY of the ledger
  Cryptography ensures all copies match
  No single point of control
```

This is **blockchain**.

---

## What is a Blockchain? {#what-is-blockchain}

A **blockchain** is a chain of blocks, where each block contains:
1. A list of transactions
2. A link to the previous block (via hash)
3. Proof of work (in Bitcoin's case)

### Visual Representation

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│ Block 1         │     │ Block 2         │     │ Block 3         │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ Prev: 0000...   │←────│ Prev: 7d3f...   │←────│ Prev: 2b8a...   │
│ Transactions:   │     │ Transactions:   │     │ Transactions:   │
│  - Alice→Bob $5 │     │  - Bob→Carol $3 │     │  - Carol→Dave $2│
│  - ...          │     │  - ...          │     │  - ...          │
│ Hash: 7d3f...   │     │ Hash: 2b8a...   │     │ Hash: 9c1e...   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
       ↑                                               ↑
   Genesis Block                               Latest Block
   (first block)
```

### Why "Chain"?

Each block contains the hash of the previous block. This creates a chain:

```
Block 2's "Previous Hash" field = Hash(Block 1)
Block 3's "Previous Hash" field = Hash(Block 2)
...
```

If anyone tries to modify Block 1, its hash changes, which breaks Block 2's link, which breaks Block 3's link, and so on. **The entire chain after the modification becomes invalid.**

### Why This Prevents Fraud

```
Attacker wants to change Block 100 to give themselves more money.

1. Attacker modifies Block 100
2. Hash of Block 100 changes
3. Block 101's "Previous Hash" no longer matches
4. Block 101 is now invalid
5. Blocks 102, 103, ..., current are all invalid
6. Everyone's copy rejects the modified chain
7. Attack fails
```

---

## Bitcoin Explained {#bitcoin}

### What is Bitcoin?

Bitcoin is:
1. **A digital currency** (BTC)
2. **A payment network** (the blockchain)
3. **A protocol** (the rules)

Created by Satoshi Nakamoto in 2008, launched in 2009.

### Key Properties

| Property | Explanation |
|----------|-------------|
| **Decentralized** | No single authority controls it |
| **Limited supply** | Only 21 million BTC will ever exist |
| **Pseudonymous** | Addresses, not names, are visible |
| **Immutable** | Past transactions can't be changed |
| **Permissionless** | Anyone can participate |

### How Bitcoin Transactions Work

```
Alice wants to send 1 BTC to Bob:

1. Alice creates transaction:
   "Send 1 BTC from address ABC to address XYZ"

2. Alice signs with her private key:
   signature = Sign(Alice_private_key, transaction)

3. Alice broadcasts to network:
   [transaction, signature, Alice_public_key]

4. Network verifies:
   - Does Alice own address ABC? (verify signature)
   - Does ABC have enough balance? (check ledger)

5. If valid, miners include in next block

6. Once in a block, transaction is confirmed
```

### Bitcoin's Cryptography

| Component | Cryptography |
|-----------|--------------|
| Private keys | Random 256-bit numbers |
| Public keys | ECDSA on secp256k1 curve |
| Addresses | Hash(Hash(public key)) |
| Signatures | ECDSA signatures |
| Block hashing | SHA-256 |
| Mining | SHA-256 puzzles |

---

## Ethereum Explained {#ethereum}

### Beyond Money: Programmable Blockchain

Bitcoin does one thing: transfer value.

Ethereum (launched 2015) added **smart contracts**: programs that run on the blockchain.

```
Bitcoin:  "Send 1 BTC to Bob"

Ethereum: "Send 1 ETH to Bob IF the date is after January 1, 2025
           AND Alice has not cancelled within 30 days"
```

### Smart Contracts

A smart contract is code that:
1. Lives on the blockchain
2. Executes automatically when conditions are met
3. Can't be stopped or modified once deployed

**Example:** Crowdfunding

```solidity
// If goal reached by deadline, send funds to project
// If not, refund all contributors

contract Crowdfund {
    if (totalRaised >= goal && now <= deadline) {
        send(project, totalRaised);
    } else if (now > deadline) {
        refundAll();
    }
}
```

### Ethereum vs Bitcoin

| Feature | Bitcoin | Ethereum |
|---------|---------|----------|
| Purpose | Digital money | Programmable platform |
| Language | Script (limited) | Solidity (Turing-complete) |
| Block time | ~10 minutes | ~12 seconds |
| Supply | 21 million max | No hard cap |
| Signatures | ECDSA | ECDSA (transitioning to post-quantum) |

### ERC-20 Tokens

Ethereum lets anyone create their own tokens using the ERC-20 standard. This enabled:
- Stablecoins (USDC, DAI)
- Governance tokens
- NFTs (ERC-721)
- DeFi protocols

---

## Wallets and Addresses {#wallets}

### What is a Wallet?

A **wallet** is software that:
1. Generates and stores your private keys
2. Derives addresses from public keys
3. Signs transactions
4. Broadcasts transactions to the network

**Important:** The wallet doesn't "hold" your coins. Your coins exist on the blockchain. The wallet holds the keys that prove you own them.

### Private Keys

A private key is just a very large random number:

```
Private key (256 bits):
0x3a1076bf45ab87712ad64ccb3b10217737f7faacbf2872e88fdd9a537d8fe266

This is YOUR SECRET. Anyone with this key can spend your coins.
```

### Public Keys

Derived from the private key using elliptic curve mathematics:

```
Private key → (elliptic curve math) → Public key

The math only works one direction:
- Easy: private key → public key
- Impossible: public key → private key (without quantum computer)
```

### Addresses

An address is a shortened, hashed version of the public key:

```
Bitcoin:
Public key → SHA-256 → RIPEMD-160 → Base58 encoding
Result: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

Ethereum:
Public key → Keccak-256 → Take last 20 bytes → Hex encoding
Result: 0x742d35Cc6634C0532925a3b844Bc9e7595f...
```

### Why Hash the Public Key?

1. **Shorter:** Addresses are more compact than full public keys
2. **Privacy:** Public key isn't revealed until you spend
3. **Security:** Extra protection layer

**But here's the quantum problem:** When you spend, you reveal your public key. A quantum computer could then derive your private key and steal any remaining funds at that address.

---

## Transactions in Detail {#transactions}

### Anatomy of a Bitcoin Transaction

```
┌─────────────────────────────────────────────────┐
│ Transaction                                      │
├─────────────────────────────────────────────────┤
│ Inputs:                                          │
│   - Previous TX: abc123, Output #0               │
│   - ScriptSig: [signature, public_key]           │
│                                                  │
│ Outputs:                                         │
│   - Output #0: 0.5 BTC to address XYZ            │
│   - Output #1: 0.4 BTC to address ABC (change)   │
│                                                  │
│ Fee: 0.1 BTC (input total - output total)        │
└─────────────────────────────────────────────────┘
```

### The UTXO Model (Bitcoin)

Bitcoin uses **Unspent Transaction Outputs (UTXOs)**:

```
Alice has received:
  TX1: 0.5 BTC
  TX2: 0.3 BTC
  TX3: 0.2 BTC
  Total: 1.0 BTC

Alice wants to send 0.7 BTC to Bob:

Inputs: TX1 (0.5) + TX2 (0.3) = 0.8 BTC
Outputs:
  - Bob: 0.7 BTC
  - Alice (change): 0.09 BTC
  - Fee: 0.01 BTC

TX1 and TX2 are now "spent" (can't be used again)
```

### The Account Model (Ethereum)

Ethereum uses accounts with balances (like a bank):

```
Alice's account: 1.0 ETH

Transaction: Send 0.7 ETH to Bob

After:
  Alice's account: 0.29 ETH (minus gas fee)
  Bob's account: +0.7 ETH
```

### Why Signatures Are Essential

Without signatures, anyone could create transactions spending your money:

```
UNSIGNED: "Send 1 BTC from Alice to Attacker"

Network: "This isn't signed. Rejected."

SIGNED: "Send 1 BTC from Alice to Bob" + Alice's signature

Network: "Signature valid. Alice authorized this. Accepted."
```

---

## Mining and Consensus {#mining}

### The Double-Spend Problem Redux

What if Alice creates two conflicting transactions?

```
TX1: Alice sends 1 BTC to Bob (for a car)
TX2: Alice sends 1 BTC to herself (fraud!)

Both are validly signed. Which one is "real"?
```

### Proof of Work (Bitcoin)

Bitcoin uses **Proof of Work (PoW)** to decide which transactions go in blocks:

```
Miners compete to find a number (nonce) such that:
  Hash(block_header, nonce) < target

This requires trillions of guesses.
First miner to find valid nonce wins the block reward.
```

The winning miner chooses which transactions to include. Once in a block with sufficient confirmations, a transaction is considered final.

### Proof of Stake (Ethereum)

Ethereum now uses **Proof of Stake (PoS)**:

```
Validators stake ETH as collateral.
Validators are randomly selected to propose blocks.
If they propose invalid blocks, their stake is slashed (taken).
```

More energy efficient than PoW, but different security assumptions.

### Consensus

**Consensus** means all nodes agree on the state of the blockchain:

```
Node A: "Block 1000 hash is abc123"
Node B: "Block 1000 hash is abc123"
Node C: "Block 1000 hash is abc123"
...
All agree = consensus reached
```

---

## Why Security Matters {#security}

### Irreversibility

Blockchain transactions are (practically) irreversible:

```
Once confirmed:
  - Can't be undone
  - Can't be modified
  - Permanent public record
```

This is a feature (prevents fraud) but also a risk (mistakes are permanent).

### Common Attack Vectors

| Attack | Description | Mitigation |
|--------|-------------|------------|
| **Private key theft** | Attacker steals your key | Hardware wallets, secure storage |
| **Phishing** | Trick user into revealing key | Education, careful verification |
| **51% attack** | Control majority of mining power | Decentralization, PoS |
| **Smart contract bugs** | Exploit code vulnerability | Audits, formal verification |
| **Quantum attack** | Break ECDSA with quantum computer | Post-quantum cryptography |

### The Quantum Threat to Blockchain

Current blockchain security rests on:

1. **Hash functions** - SHA-256, Keccak (quantum-resistant)
2. **ECDSA signatures** - Based on elliptic curves (**quantum-vulnerable!**)

```
Today:
  Public key → (impossible) → Private key

With Quantum Computers:
  Public key → (Shor's algorithm) → Private key in hours
```

**At risk:**
- Any address that has ever sent a transaction (public key exposed)
- $718 billion in Bitcoin in vulnerable addresses
- All Ethereum accounts that have ever transacted

**This is why post-quantum cryptography (like WOTS+) matters!**

---

## Key Takeaways {#takeaways}

1. **Blockchain** solves double-spending without trusted authorities

2. **Bitcoin** is digital money; **Ethereum** is a programmable platform

3. **Wallets** store private keys, not coins

4. **Addresses** are derived from public keys via hashing

5. **Signatures** prove you authorized a transaction

6. **Mining/staking** creates consensus on transaction ordering

7. **Immutability** means mistakes and thefts are permanent

8. **ECDSA** (current signatures) is vulnerable to quantum computers

9. **Post-quantum cryptography** is needed before quantum computers arrive

---

## Glossary {#glossary}

| Term | Definition |
|------|------------|
| **Blockchain** | Distributed ledger of linked blocks |
| **Block** | Group of transactions with metadata |
| **Transaction (TX)** | Transfer of value or data |
| **Wallet** | Software managing private keys |
| **Address** | Public identifier for receiving funds |
| **Private key** | Secret needed to spend funds |
| **Public key** | Derived from private key, used for verification |
| **Mining** | Process of creating new blocks (PoW) |
| **Staking** | Locking funds as collateral (PoS) |
| **Consensus** | Agreement on blockchain state |
| **UTXO** | Unspent Transaction Output |
| **Smart contract** | Self-executing code on blockchain |
| **Gas** | Ethereum's unit of computation cost |
| **Confirmation** | Number of blocks after a transaction |

---

## What's Next?

- **[03-BEGINNER-QUANTUM](03-BEGINNER-QUANTUM.md)** - Understand quantum computing threat
- **[05-INTERMEDIATE-DIGITAL-SIGNATURES](05-INTERMEDIATE-DIGITAL-SIGNATURES.md)** - Deep dive into ECDSA
- **[06-INTERMEDIATE-WOTS](06-INTERMEDIATE-WOTS.md)** - Learn about quantum-resistant signatures

---

## Self-Check Questions

1. What problem does blockchain solve?
2. What's the difference between Bitcoin and Ethereum?
3. Why is the public key only revealed when you spend?
4. What makes blockchain immutable?
5. Why are current signatures vulnerable to quantum computers?

*If you can answer these, you're ready for the next tutorial!*
