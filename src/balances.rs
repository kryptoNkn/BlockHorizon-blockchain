use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { balances: BTreeMap::new() }
    }

    // set the balance of an account `who` to some `amount`
    pub fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    // get the balance of an account `who`
    pub fn balance(&self, who: &str) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    // transfer `amount` from one account to another
    // this function verifies that `from` has at least `amount` balance to transfer
    // and that no mathematical overflows occur
    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: u128,
    ) -> Result<(), &'static str> {
        let from_balance = self.balance(from);
        let to_balance = self.balance(to);

        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding to balance")?;

        // update balances
        self.set_balance(from, new_from_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balance() {
        let mut pallet = Pallet::new();

        assert_eq!(pallet.balance("alice"), 0);
        pallet.set_balance("alice", 100);
        assert_eq!(pallet.balance("alice"), 100);
        assert_eq!(pallet.balance("bob"), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut pallet = Pallet::new();

        let alice = "alice";
        let bob = "bob";

        assert_eq!(pallet.balance(alice), 0);
        pallet.set_balance(alice, 100);

        pallet.transfer(alice, bob, 90).unwrap();

        assert_eq!(pallet.balance(alice), 10);
        assert_eq!(pallet.balance(bob), 90);
    }
}
