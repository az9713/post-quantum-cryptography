# Quick Start Guide: 10 Hands-On Examples

This guide gives you quick wins to understand post-quantum cryptography through hands-on examples. Each example takes 1-2 minutes and teaches a specific concept.

## Before You Start

### Step 1: Open Your Terminal

**Windows (Git Bash recommended):**
1. Press `Win + S`, type "Git Bash", press Enter
2. Or use PowerShell/Command Prompt

**Mac:**
1. Press `Cmd + Space`, type "Terminal", press Enter

**Linux:**
1. Press `Ctrl + Alt + T`

### Step 2: Clone and Navigate to the Project

```bash
# Clone the repository
git clone https://github.com/az9713/post-quantum-cryptography.git

# Navigate into the project
cd post-quantum-cryptography
```

### Step 3: Verify Rust is Installed

```bash
cargo --version
```

You should see something like: `cargo 1.93.0`

If not, install Rust:
```bash
# Windows
winget install Rustlang.Rustup

# Mac/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Example 1: Run the Full Demo

**Goal:** See everything the application can do

**Command:**
```bash
cargo run
```

**What You'll Learn:**
- The quantum threat to blockchain
- How WOTS+ signatures work
- Why one-time signatures matter
- Size comparison with current cryptography

**Expected Output:** Colorful educational display with live demonstrations

**Time:** ~30 seconds (first run compiles, subsequent runs are instant)

---

## Example 2: Run All Tests

**Goal:** Verify everything works correctly

**Command:**
```bash
cargo test
```

**What You'll Learn:**
- The project has 48 automated tests
- All cryptographic operations are verified
- Security properties are enforced

**Expected Output:**
```
test result: ok. 48 passed; 0 failed
```

**Time:** ~5 seconds

---

## Example 3: Test WOTS+ Key Generation

**Goal:** Verify that quantum-safe keys are generated correctly

**Command:**
```bash
cargo test test_keypair_generation
```

**What You'll Learn:**
- Keys are 2144 bytes (vs 33 bytes for ECDSA)
- Keys start as "unused"
- Generation is fast (~2ms)

**Expected Output:**
```
test test_keypair_generation ... ok
```

---

## Example 4: Test the One-Time Property

**Goal:** See the critical security feature in action

**Command:**
```bash
cargo test test_one_time_property
```

**What You'll Learn:**
- First signature succeeds
- Second signature is **blocked**
- This prevents signature forgery attacks

**Why This Matters:**
In WOTS+, signing twice reveals enough information for attackers to forge signatures. This test proves our implementation blocks that attack.

**Expected Output:**
```
test test_one_time_property ... ok
```

---

## Example 5: Test Signature Verification

**Goal:** Confirm signatures can be verified

**Command:**
```bash
cargo test test_sign_and_verify
```

**What You'll Learn:**
- Messages are hashed before signing
- Signatures are ~2144 bytes
- Verification confirms the signer had the private key

**Expected Output:**
```
test test_sign_and_verify ... ok
```

---

## Example 6: Test Tamper Detection

**Goal:** See what happens when someone modifies a signed transaction

**Command:**
```bash
cargo test test_tampered_transaction_fails
```

**What You'll Learn:**
- Signatures cover ALL transaction data
- Changing any field invalidates the signature
- Attackers cannot modify transactions after signing

**The Test Does This:**
1. Creates a transaction for 1000 units
2. Signs it
3. Changes amount to 9999
4. Verification **fails** (as expected)

**Expected Output:**
```
test test_tampered_transaction_fails ... ok
```

---

## Example 7: Test Wrong Key Detection

**Goal:** Verify that only the correct key can sign

**Command:**
```bash
cargo test test_wrong_keypair_signing_fails
```

**What You'll Learn:**
- You cannot sign a transaction from someone else's address
- The system checks that the keypair matches the sender

**Expected Output:**
```
test test_wrong_keypair_signing_fails ... ok
```

---

## Example 8: Test Address Generation

**Goal:** See how blockchain addresses are derived

**Command:**
```bash
cargo test test_address_from_keypair
```

**What You'll Learn:**
- Addresses start with "qw1" (quantum-winternitz-1)
- Addresses are derived by hashing the public key
- Same keypair always produces same address

**Expected Output:**
```
test test_address_from_keypair ... ok
```

---

## Example 9: Test Signature Serialization

**Goal:** Verify signatures can be saved and loaded

**Command:**
```bash
cargo test test_signature_serialization
```

**What You'll Learn:**
- Signatures can be converted to bytes for storage
- Signatures can be recovered from bytes
- Recovered signatures still verify correctly

**Why This Matters:**
In real blockchains, signatures must be stored in blocks and transmitted over networks.

**Expected Output:**
```
test test_signature_serialization ... ok
```

---

## Example 10: Run Multiple Transaction Test

**Goal:** See multiple independent transactions work

**Command:**
```bash
cargo test test_multiple_transactions
```

**What You'll Learn:**
- Each transaction uses a fresh keypair
- 5 independent transactions all succeed
- The system scales correctly

**Expected Output:**
```
test test_multiple_transactions ... ok
```

---

## Bonus: Run Tests with Verbose Output

**Goal:** See detailed test execution

**Command:**
```bash
cargo test -- --nocapture
```

This shows any `println!` output from tests.

---

## Bonus: Build Optimized Version

**Goal:** Create a fast production build

**Command:**
```bash
cargo build --release
```

The optimized binary is at: `target/release/pqc-demo.exe` (Windows) or `target/release/pqc-demo` (Mac/Linux)

---

## Summary: What You've Learned

| Example | Concept |
|---------|---------|
| 1 | Full demonstration of quantum-safe cryptography |
| 2 | All 48 tests pass |
| 3 | Key generation creates 2144-byte keys |
| 4 | One-time property blocks second signatures |
| 5 | Signatures verify correctly |
| 6 | Tampered transactions are rejected |
| 7 | Wrong keys cannot sign |
| 8 | Addresses derived from public keys |
| 9 | Signatures serialize/deserialize |
| 10 | Multiple transactions work independently |

---

## Sample Outputs

Can't run the commands? Check out the pre-captured outputs:

- **[Demo Output](examples/demo_results.txt)** - Full `cargo run` output showing educational content
- **[Test Results](examples/test_results.txt)** - Complete `cargo test` output with all 48 passing tests

---

## Next Steps

Now that you have quick wins, explore deeper:

1. **Read the User Guide** - [USER_GUIDE.md](USER_GUIDE.md)
2. **Understand the Architecture** - [ARCHITECTURE.md](ARCHITECTURE.md)
3. **Modify the Code** - [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md)
4. **Learn the Theory** - [Tutorials](../tutorials/00-INDEX.md) (beginner to PhD level)

---

## Troubleshooting

### "cargo: command not found"

Rust isn't installed or not in PATH. Solutions:

**Windows (Git Bash):**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Or reinstall Rust:**
```bash
winget install Rustlang.Rustup
```
Then restart your terminal.

### "error[E0433]: failed to resolve"

Run `cargo build` first to download dependencies.

### Tests fail

Make sure you're in the correct directory:
```bash
pwd
# Should show: .../post-quantum-cryptography
```

### Slow first run

The first `cargo run` or `cargo test` compiles the project. Subsequent runs are fast.
