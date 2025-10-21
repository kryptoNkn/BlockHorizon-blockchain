use std::collections::BTreeMap;
use crate::balances;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    fn new() -> Self {
        Self {
             balances:  BTreeMap::new(),
        }
    }
}