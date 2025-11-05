use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer, Verifier};
use rand::rngs::OsRng;
use rand::RngCore;

pub struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let mut key_bytes = [0u8; 32];

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let verifying_key = signing_key.verifying_key();

        Wallet { signing_key, verifying_key }
    }

    pub fn address(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.signing_key.sign(msg)
    }

    pub fn verify(&self, msg: &[u8], sig: &Signature) -> bool {
        self.verifying_key.verify(msg, sig).is_ok()
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.verifying_key
    }
}
