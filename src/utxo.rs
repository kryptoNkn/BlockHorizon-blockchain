use std::collections::HashMap;
use crate::Hash;
use crate::transaction::Output;

#[derive(Debug, Clone)]
pub struct UTXOSet {
    pub unspent: HashMap<Hash, Output>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet { unspent: HashMap::new(), }
    }
}

