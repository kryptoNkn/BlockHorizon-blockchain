use super::*;
use std::collections::HashSet;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use crate::wallet::Wallet;

#[derive(Debug, Clone)]
pub struct Output {
    pub to_addr: String,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn sign(&mut self, wallet: &Wallet) {
        let message = self.bytes();
        self.signature = Some(wallet.sign(&message));
    }

    pub fn verify(&self, pubkey: &VerifyingKey) -> bool {
        if let Some(sig) = &self.signature {
            pubkey.verify(&self.bytes(), sig).is_ok()
        } else {
            false
        }
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|i| i.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|o| o.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs.iter().map(|i| i.hash()).collect()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs.iter().map(|o| o.hash()).collect()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}
