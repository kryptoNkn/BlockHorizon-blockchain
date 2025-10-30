use std::collections::HashMap;
use crate::{Hash, transaction::Output, transaction::Transaction, Hashable};

#[derive(Debug, Clone)]
pub struct UTXOSet {
    pub unspent: HashMap<Hash, Output>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            unspent: HashMap::new(),
        }
    }

    pub fn add_output(&mut self, output: &Output) {
        self.unspent.insert(output.hash(), output.clone());
    }

    pub fn remove_output(&mut self, output: &Output) {
        self.unspent.remove(&output.hash());
    }

    pub fn contains(&self, output: &Output) -> bool {
        self.unspent.contains_key(&output.hash())
    }

    // Applying a transaction to a UTXO: checking for inputs and updating the set
    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        if tx.is_coinbase() {
            for out in &tx.outputs {
                self.add_output(out);
            }
            return Ok(());
        }

        // For non-empty transactions: signature verification and input ownership by the sender
        if !tx.verify() {
            return Err("Invalid transaction signature or owner mismatch");
        }

        for input in &tx.inputs {
            if !self.contains(input) {
                return Err("Attempt to spend non-existing or already spent output");
            }
        }

        
        for input in &tx.inputs {
            self.remove_output(input);
        }

        for out in &tx.outputs {
            self.add_output(out);
        }

        Ok(())
    }
}
