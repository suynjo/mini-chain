use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(genesis_block: Block, difficulty: usize) -> Self {
        Self {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        let target = "0".repeat(self.difficulty);

        for i in 0..self.chain.len() {
            let current = &self.chain[i];

            if !current.is_valid() {
                return false;
            }

            if !current.hash.starts_with(&target) {
                return false;
            }

            if i > 0 {
                let previous = &self.chain[i - 1];

                if current.previous_hash != previous.hash {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(&self) {
        println!("Blockchain valid: {}", self.is_valid());
    }
}