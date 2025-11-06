# 🪙 BlockHorizon — Minimal Blockchain Implementation in Rust

**BlockHorizon** is a minimal blockchain prototype written entirely in **Rust**.  
It implements the core building blocks of blockchain technology:
- Proof-of-Work (mining),
- UTXO transaction model,
- digital signatures (Ed25519),
- wallet system,
- full blockchain validation.

This project focuses on **clarity, correctness, and architecture**, avoiding heavy dependencies like `serde` or async runtimes.  
It’s designed as a learning-oriented yet technically solid blockchain core.

---

## Features

- Wallet creation and digital signing  
- Transaction verification and validation  
- Proof-of-Work mining  
- Blockchain validation with difficulty check  
- UTXO (Unspent Transaction Outputs) model  
- Automatic balance updates on new blocks  
- Ed25519 cryptography for signatures  
- Basic unit tests

---

## Project Structure

```
src/  
├── block.rs # Block structure and Proof-of-Work  
├── blockchain.rs # Chain validation and block addition  
├── hashable.rs # Hashing traits for all structures  
├── transaction.rs # Transaction logic and signatures  
├── utxo.rs # UTXO set management  
├── wallet.rs # Wallet and address generation  
├── lib.rs # Utilities and type aliases  
├── main.rs # Demo of blockchain execution  
└── unit_tests.rs # Basic tests
```

## How to run

```bash
git clone https://github.com/yourusername/blockhorizon.git
cd blockhorizon
cargo run
```

You’ll see the mining process of the genesis block and the creation of a second block with UTXO updates:

```
Mined genesis: Block[0]: 00a3f9... with 2 transactions
Mined block: Block[1]: 0001d3... with 2 transactions
UTXO set:
ef6c... -> alice (value: 40)
cfa7... -> bob (value: 17)
...
```

## Key Concepts
### Proof of Work (PoW)

Blocks are mined by incrementing a `nonce` until the hash satisfies a given difficulty target.

```rust
pub fn mine(&mut self) {
    for nonce_attempt in 0..u64::MAX {
        self.nonce = nonce_attempt;
        let hash = self.hash();
        if check_difficulty(&hash, self.difficulty) {
            self.hash = hash;
            return;
        }
    }
}
```

### Wallets and Signatures

Each wallet generates an Ed25519 keypair and can sign transactions.

```rust
let alice = Wallet::new();
let bob = Wallet::new();

let mut tx = Transaction {
    inputs: vec![alice_output.clone()],
    outputs: vec![Output { to_addr: bob.address(), value: 10 }],
    signature: None,
    owner_pubkey: None,
};

tx.sign(&alice);
assert!(tx.verify());
```

### Testing

Run the built-in unit tests:

```bash
cargo test
```

Unit tests cover:
- wallet generation and signature verification,
- mining and difficulty validation,
- blockchain integrity checks,
- UTXO updates after transactions.

## Author

**BlockHorizon** was created as a personal learning project to explore:

- system programming with **Rust**,
- blockchain architecture (PoW, UTXO, cryptographic signatures),
- practical cryptography (Ed25519, SHA256),
- modular software design.
