#[cfg(test)]
mod tests {
    #[test]
    fn test_wallet_address_and_signing() {
        let wallet = Wallet::new();
        let message = b"message :)";
        let sig = wallet.sign(message);
        assert!(wallet.verify(message, &sig));
        assert_eq!(wallet.address().len(), 64);
    }

    #[test]
    fn test_genesis_block_mining() {
        let difficulty = 0x00ffffffffffffffffffffffffffffffff;
        let genesis_tx = Transaction {
            inputs: vec![],
            outputs: vec![transaction::Output {
                to_addr: "addr_!".to_string(),
                volue: 50,
            }],
            signature: None,
            owner_pubkey: None,
        };
        let mut block = Block::new(0, now(), vec![0; 32], vec![genesis_tx], difficulty);
        block.mine();
        assert!(blockchain::check_difficulty(&block.hash, block.difficulty));
    }

    #[test]
    fn test_transaction_sign_and_verify() {
        let alice = Wallet::new();
        let bob = Wallet::new();
        let mut tx = Transaction {
            inputs: vec![],
            outputs: vec![transaction::Output {
                to_addr: bob.address(),
                value: 10,
            }],
            signature: None,
            owner_pubkey: None,
        };
        tx.sign(&alice);
        assert!(tx.verify());
        assert!(tx.is_coinbase()); 
    }

    #[test]
    fn test_utxo_apply_transaction() {
        let mut utxo_set = UTXOSet::new();
        let alice = Wallet::new();
        let bob = Wallet::new();

        let output = transaction::Output {
            to_addr: alice.address(),
            value: 50,
        };

        utxo_set.add_output(&output);
        assert!(utxo_set.contains(&output));

        let mut tx = Transaction {
            inputs: vec![output.clone()],
            outputs: vec![
                transaction::Output { to_addr: bob.address(), value: 30 },
                transaction::Output { to_addr: alice.address(), value: 20 },
            ],
            signature: None,
            owner_pubkey: None,
        };

        tx.sign(&alice);
        assert!(utxo_set.apply_transaction(&tx).is_ok());
        assert_eq!(utxo_set.unspent.len(), 2);
    }
}