use std::sync::{Arc, Mutex};
use crate::{Blockchain, Block, Transaction};

pub enum Message {
    NewBlock(Block),
    NewTransaction(Transaction),
    RequestChain, // request for a new chain
    ChainResponse(Vec<Block>), // answer by chain of blocks
}

pub struct P2PNode {
    peers: Vec<String>, // list of addresses of other nodes
    blockchain: Arc<Mutex<Blockchain>>, // public access to the blockchain
}