use super::*;

pub struct Output {
    pub addr: Address,
    pub value: u64,
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}