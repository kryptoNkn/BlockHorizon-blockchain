use super::*;
use crate::block::check_difficulty;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub utxos: UTXOSet,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![],
            utxos: UTXOSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), &'static str> {
        if let Some(prev_block) = self.blocks.last() {
            if block.index != prev_block.index + 1 {
                return Err("Invalid index");
            }
            if block.prev_block_hash != prev_block.hash {
                return Err("Invalid previous hash");
            }
        }

        if !check_difficulty(&block.hash, block.difficulty) {
            return Err("Block hash does not satisfy difficulty");
        }

        let mut temp_utxos = self.utxos.clone();
        for tx in &block.transactions {
            temp_utxos.apply_transaction(tx).map_err(|_| "Invalid transaction in block")?;
        }

        self.blocks.push(block);
        self.utxos = temp_utxos;
        Ok(())
    }
}
