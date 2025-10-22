mod balances;
use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");
    let mut pallet = balances::Pallet::new();

    let number: u64 = 1;
    let number2 = number.checked_sub(Some(10).unwrap());
    println!("number2 = {}", number2.unwrap_or(0));
}
 