pub enum Message {
    NewBlock(Block),
    NewTransaction(Transaction),
    RequestChain,
    ChainResponse(Vec<Block>),
}