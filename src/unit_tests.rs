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
}