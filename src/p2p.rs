use std::{net::TcpListener, sync::{Arc, Mutex}};
use serde::{Serialize, Deserialize};
use tokio::time::Sleep;
use crate::Blockchain;

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Chain(Vec<String>),
    NewBlock(String),
}

pub struct P2PNode {
    pub peers: Arc<Mutex<Vec<String>>>,
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl P2PNode {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        Self {
            peers: Arc::new(Mutex::new(vec![])),
            blockchain,
        }
    }
}

// pub async fn start(&self, addr: &str) -> tokio::io::Result<()> {
//     let listener = TcpListener::bind(addr).await?;

// }