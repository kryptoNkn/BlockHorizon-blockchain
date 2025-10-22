use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    // set the balance of an account `who` to some `amount`
    pub fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who, amount);
    }

    // get the balance of an account `who`
    pub fn balance(&self, who: String) -> u128 {
        *self.balances.get(&who).unwrap_or(&0)
    }

    // transfer `amount` from one account to another
    // this function verifies that `from` has at least `amount` balance to transfer
    // and that no mathematical overflows occur
    pub fn transfer(
        &mut self, 
        caller: String, 
        to: String, 
        amount: u128
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
        .checked_sub(amount)
        .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
        .checked_sub(amount)
        .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}