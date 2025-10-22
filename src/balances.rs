use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // set the balance of an account `who` to some `amount`
    pub fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who, amount);
    }

    // get the balance of an account `who`
    pub fn balance(&self, who: String) -> u128 {
        *self.balances.get(&who).unwrap_or(&0)
    }
}
