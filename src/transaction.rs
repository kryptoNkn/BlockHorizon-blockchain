use super::*;
use std::collections::HashSet;
use ed25519_dalek::{Signature, VerifyingKey, Signer, Verifier};
use crate::wallet::Wallet;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
    pub signature: Option<Signature>,
    pub owner_pubkey: Option<VerifyingKey>,
}

impl Transaction {
    pub fn sign(&mut self, wallet: &Wallet) {
        let message = self.bytes();
        let sig = wallet.sign(&message);
        self.signature = Some(sig);
        self.owner_pubkey = Some(wallet.public_key());
    }

    /// verification of signature and ownership of inputs by the owner
    pub fn verify(&self) -> bool {
        if self.is_coinbase() {
            return true;
        }

        let pubkey = match &self.owner_pubkey {
            Some(pk) => pk,
            None => return false,
        };
        let sig = match &self.signature {
            Some(s) => s,
            None => return false,
        };

        if pubkey.verify(&self.bytes(), sig).is_err() {
            return false;
        }

        let owner_addr = hex::encode(pubkey.to_bytes());
        for inp in &self.inputs {
            if inp.to_addr != owner_addr {
                return false;
            }
        }

        true
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|i| i.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|o| o.value).sum()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs.iter().map(|i| i.hash()).collect()
    }

    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs.iter().map(|o| o.hash()).collect()
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

        if let Some(pk) = &self.owner_pubkey {
            bytes.extend(&pk.to_bytes());
        }

        bytes
    }
}
