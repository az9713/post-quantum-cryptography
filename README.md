# Post-Quantum Cryptography Demo

An educational Rust application demonstrating **WOTS+ (Winternitz One-Time Signatures)** - a quantum-resistant signature scheme that could protect blockchain systems against future quantum computer attacks.

## What This Project Does

This project implements a **production-quality WOTS+ cryptographic library** with educational demonstrations. It shows how the signature layer of blockchain systems (Bitcoin, Ethereum) could be upgraded to resist quantum attacks.

**Important:** This is a **cryptography demo**, not a blockchain implementation. See [Scope & Limitations](#scope--limitations) for details.

## Why This Matters

- **Current Problem**: Bitcoin and Ethereum use ECDSA signatures, which quantum computers will break
- **Timeline**: Experts estimate "Q-Day" (when quantum breaks crypto) between 2030-2040
- **Scale**: ~$718 billion in Bitcoin sits in vulnerable addresses
- **This Solution**: WOTS+ signatures are quantum-resistant because they only rely on hash functions

## Scope & Limitations

### What's Implemented (Realistic)

| Component | Status | Notes |
|-----------|--------|-------|
| **WOTS+ signatures** | ✅ Full | Correct algorithm per RFC 8391 |
| **Hash functions** | ✅ Full | Blake3, SHA3-256 (production-grade) |
| **Key generation** | ✅ Full | Cryptographically secure RNG |
| **Transaction signing** | ✅ Full | Proper digest → sign → verify flow |
| **One-time enforcement** | ✅ Full | Critical security property enforced |
| **Address derivation** | ✅ Full | Hash of public key (like Ethereum) |

### What's NOT Implemented (Simulation Only)

| Component | Status | What Real Blockchains Have |
|-----------|--------|----------------------------|
| **Blocks** | ❌ None | Chain of blocks with headers, Merkle roots |
| **Chain structure** | ❌ None | Previous block hash linking |
| **Consensus** | ❌ None | Proof-of-Work, Proof-of-Stake, etc. |
| **P2P Network** | ❌ None | Node discovery, gossip protocol |
| **State management** | ❌ None | Account balances, UTXO tracking |
| **Mempool** | ❌ None | Pending transaction queue |
| **Persistence** | ❌ None | Database storage |
| **Smart contracts** | ❌ None | EVM, scripting languages |

### Architecture Comparison

```
REAL BLOCKCHAIN                    THIS DEMO
───────────────                    ─────────
┌─────────────┐
│   Network   │ ← P2P gossip       (not implemented)
└──────┬──────┘
       ▼
┌─────────────┐
│   Mempool   │ ← Pending txs      (not implemented)
└──────┬──────┘
       ▼
┌─────────────┐
│  Consensus  │ ← PoW/PoS          (not implemented)
└──────┬──────┘
       ▼
┌─────────────┐                    ┌─────────────┐
│    Block    │ ← Merkle trees     │ Transaction │ ← Just this layer
└──────┬──────┘                    └──────┬──────┘
       ▼                                  ▼
┌─────────────┐                    ┌─────────────┐
│ Transaction │ ← Sign/Verify      │  WOTS+ Sig  │ ← Fully implemented
└─────────────┘                    └─────────────┘
```

**Bottom line:** The cryptography is production-quality. The "blockchain" is just transaction objects for demonstration purposes.

## Quick Start (5 Minutes)

### Prerequisites

1. **Install Rust** (if not already installed):
   - Windows: Download from [rustup.rs](https://rustup.rs/) or run:
     ```
     winget install Rustlang.Rustup
     ```
   - Mac/Linux:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Restart your terminal** after installing Rust

### Run the Demo

```bash
# Navigate to project
cd post-quantum-cryptography

# Run the educational demo
cargo run

# Run all tests
cargo test
```

## What You'll See

The demo displays:

1. **Educational explanations** of quantum threats
2. **Live key generation** with timing (~2ms)
3. **Transaction signing** demonstration (~200μs)
4. **One-time property enforcement** (second signature blocked)
5. **Size comparison** (WOTS+ is ~34x larger than ECDSA)

## Documentation

| Document | Audience | Description |
|----------|----------|-------------|
| [Quick Start Guide](docs/QUICK_START.md) | Everyone | 10 hands-on examples to learn by doing |
| [User Guide](docs/USER_GUIDE.md) | Users | Complete guide to using this application |
| [Developer Guide](docs/DEVELOPER_GUIDE.md) | Developers | How to modify and extend the code |
| [Architecture](docs/ARCHITECTURE.md) | Developers | Technical design and code structure |
| [Tutorials](tutorials/00-INDEX.md) | All Levels | Comprehensive tutorials from beginner to PhD level |
| [CLAUDE.md](CLAUDE.md) | AI Assistants | Instructions for Claude Code |

### Sample Outputs

Pre-captured outputs are available in `docs/examples/`:
- [Demo Output](docs/examples/demo_results.txt) - Full `cargo run` output with colored educational content
- [Test Results](docs/examples/test_results.txt) - Complete `cargo test` output showing all 48 passing tests

## Project Structure

```
post-quantum-cryptography/
├── src/
│   ├── main.rs          # Demo entry point
│   ├── lib.rs           # Library exports
│   ├── wots/            # WOTS+ cryptography
│   ├── blockchain/      # Transaction simulation
│   └── demo/            # Educational output
├── tests/               # Integration tests
├── docs/                # User and developer documentation
│   └── examples/        # Captured demo and test outputs
├── tutorials/           # 11 tutorials (beginner to PhD level)
└── Cargo.toml           # Project configuration
```

## Key Concepts

| Term | Meaning |
|------|---------|
| **WOTS+** | Winternitz One-Time Signatures Plus - quantum-safe signature scheme |
| **Q-Day** | The day quantum computers can break current cryptography |
| **Hash Chain** | Repeatedly hashing a value (H(H(H(...H(x))))) |
| **One-Time** | Each WOTS+ key can only sign ONE message safely |
| **STARK** | Proof system that can compress many signatures into one |

## Commands Reference

| Command | What It Does |
|---------|--------------|
| `cargo run` | Run the educational demo |
| `cargo test` | Run all 48 tests |
| `cargo test --test wots_tests` | Run WOTS+ tests only |
| `cargo test --test blockchain_tests` | Run blockchain tests only |
| `cargo build` | Compile without running |
| `cargo build --release` | Compile optimized version |

## Test Results

All 48 tests pass:
- 9 WOTS+ cryptography tests
- 14 blockchain transaction tests
- 24 library unit tests
- 1 demo integration test

See [test_results.txt](docs/examples/test_results.txt) for complete output.

---

## Highlights

### Comprehensive Tutorial Series

This project includes **11 in-depth tutorials** covering the full spectrum from beginner to PhD-level content:

| Level | Topics Covered |
|-------|----------------|
| **Beginner** (3) | Cryptography basics, Blockchain fundamentals, Quantum computing introduction |
| **Intermediate** (3) | Hash functions, Digital signatures (RSA, ECDSA), WOTS+ deep dive |
| **Advanced** (2) | Shor's & Grover's algorithms, Zero-knowledge proofs & STARKs |
| **PhD** (2) | Formal cryptographic security proofs, Post-quantum cryptography landscape |

Perfect for learners from **any background** - mathematics, physics, computer science, finance, or blockchain development. See the [Tutorial Index](tutorials/00-INDEX.md) for learning paths tailored to your background.

### Verified Implementation

All functionality is verified with **48 automated tests** and **live demonstrations**:
- [Demo Output](docs/examples/demo_results.txt) - Educational walkthrough with live cryptographic operations
- [Test Results](docs/examples/test_results.txt) - Complete test suite output

---

## Acknowledgements

### Inspiration

This project was inspired by the announcement covered in a16z crypto's newsletter ["What went down in DC this week"](https://a16zcrypto.substack.com/p/what-went-down-in-dc-this-week):

> *"Last week, the Ethereum Foundation (EF) announced a new Post-Quantum team. They have 'officially declared' post-quantum security a top strategic priority, given that the pace of engineering breakthroughs have been 'nothing short of phenomenal' since EF started their R&D journey here a few years ago. 'It's now 2026, timelines are accelerating. Time to go full post-quantum', shared security researcher Justin Drake."*

### AI-Generated Content

**All code and documentation in this project were generated by [Claude Code](https://claude.ai/claude-code) powered by Claude Opus 4.5.**

This includes:
- Complete Rust implementation of WOTS+ signatures
- Blockchain transaction simulation
- Educational demo with colored terminal output
- 48 automated tests
- All documentation (README, guides, architecture docs)
- 11 comprehensive tutorials (beginner to PhD level)

This project demonstrates the capability of AI-assisted software development for creating educational, well-documented, and thoroughly tested codebases.

---

## License

MIT

## Further Reading

- [a16z Crypto: Quantum Computing and Blockchains](https://a16zcrypto.com/posts/article/quantum-computing-misconceptions-realities-blockchains-planning-migrations/)
- [Ethereum Foundation Post-Quantum Initiatives](https://www.coindesk.com/tech/2026/01/24/ethereum-foundation-makes-post-quantum-security-a-top-priority-as-new-team-forms)
- [NIST Post-Quantum Standards](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)
- [WOTS+ RFC 8391](https://datatracker.ietf.org/doc/html/rfc8391)
