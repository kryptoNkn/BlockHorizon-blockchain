mod balances;
use std::collections::BTreeMap;


fn main() {
    println!("Hello, world!");
    let mut map = BTreeMap::new();
    map.insert("alice", 100);
    assert_eq!(map.get(&"alice").unwrap_or(&0), &100);
    assert_eq!(map.get(&"bob"), None);

    let maybe_value = map.get(&"bob");
    match maybe_value {
        Some(value) => {
            println!("{value}")
        },
        None => {
            println!("no value")
        },
    }
}
