use BlockHorizonLib::*;
use crate::wallet::Wallet;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let alice = Wallet::new();
    let bob = Wallet::new();
    let miner = Wallet::new();

    let alice_addr = alice.address();
    let bob_addr = bob.address();
    let miner_addr = miner.address();

    let genesis_tx = Transaction {
        inputs: vec![],
        outputs: vec![
            transaction::Output { to_addr: alice_addr.clone(), value: 50 },
            transaction::Output { to_addr: bob_addr.clone(), value: 7 },
        ],
        signature: None,
        owner_pubkey: None,
    };

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![genesis_tx],
        difficulty,
    );

    genesis_block.mine();
    println!("Mined genesis: {:?}", genesis_block);
    let mut blockchain = Blockchain::new();
    blockchain.update_with_block(genesis_block).expect("add genesis");

    let alice_utxo = {
        let mut found: Option<transaction::Output> = None;
        for out in blockchain.utxos.unspent.values() {
            if out.to_addr == alice_addr {
                found = Some(out.clone());
                break;
            }
        }
        found.expect("Alice has no UTXO")
    };

    let mut tx = Transaction {
        inputs: vec![alice_utxo.clone()],
        outputs: vec![
            transaction::Output { to_addr: bob_addr.clone(), value: 10 },
            transaction::Output { to_addr: alice_addr.clone(), value: alice_utxo.value - 10 },
        ],
        signature: None,
        owner_pubkey: None,
    };

    tx.sign(&alice);

    let coinbase = Transaction {
        inputs: vec![],
        outputs: vec![transaction::Output { to_addr: miner_addr.clone(), value: 50 }],
        signature: None,
        owner_pubkey: None,
    };

    let mut block = Block::new(
        1,
        now(),
        blockchain.blocks.last().unwrap().hash.clone(),
        vec![coinbase, tx],
        difficulty,
    );

    block.mine();
    println!("Mined block: {:?}", block);

    blockchain.update_with_block(block).expect("add block");

    println!("UTXO set:");
    for (h, out) in &blockchain.utxos.unspent {
        println!("{} -> {:?} (value: {})", hex::encode(h), out.to_addr, out.value);
    }
}
