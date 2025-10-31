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
}