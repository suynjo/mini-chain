use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::transaction::Transaction;


pub struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn next_index(&self) -> u64 {
        self.chain.len() as u64
    }

    pub fn new(genesis_block: Block, difficulty: usize) -> Self {
        Self {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn is_valid(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        let target = "0".repeat(self.difficulty);

        for i in 0..self.chain.len() {
            let current = &self.chain[i];

            if !current.is_valid() {
                return false;
            }

            if !current.hash().starts_with(&target) {
                return false;
            }

            if i > 0 {
                let previous = &self.chain[i - 1];

                if current.previous_hash() != previous.hash() {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        println!("Blockchain valid: {}", self.is_valid());
    }

    pub fn make_block (&self, transaction: Transaction)-> Block {
        let index = self.next_index();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let transactions = transaction.to_string();
        let previous_hash = self.chain.last().unwrap().hash();
        let (nonce, hash) = Block::mine(index, timestamp, &transactions, &previous_hash, self.difficulty);

        Block::new(
            index,
            timestamp,
            &transactions,
            previous_hash,
            nonce,
            &hash,
        )
    }
}