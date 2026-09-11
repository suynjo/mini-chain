use sha2::{Digest, Sha256};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let difficulty : usize = 4;
    let genesis_block = Block::genesis(difficulty);
    let mut blockchain = Blockchain::new(genesis_block, difficulty);
    let block = blockchain.blocks.first().unwrap();
    block.print();
    blockchain.print();
    
    loop {
        let mut input = String::new();
            io::stdin()
            .read_line(&mut input)
            .unwrap();
        let data = input.trim();

        blockchain.add_block(data.to_string());

        let block = blockchain.blocks.last().unwrap();
        block.print();
        blockchain.print();
    }
}
#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

struct Blockchain {
    blocks: Vec<Block>,
    difficulty: usize,
}

impl Block {
    fn genesis(difficulty: usize) -> Self {
        let (nonce, hash) = Self::mine(0, 0, "Genesis Block", "0", difficulty,);
        Self::new(0, 0, "Genesis Block", "0", nonce, &hash)
    }

    fn new(index: u64, timestamp: u64, data: &str, previous_hash: &str, nonce: u64, hash: &str)-> Self {  
        Self {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            nonce,
            hash: hash.to_string(),
        }
    }

    fn mine(index: u64, timestamp: u64, data: &str, previous_hash: &str, difficulty: usize)-> (u64, String){
        let mut nonce = 0;
        let target = "0".repeat(difficulty);

        loop {
            let hash = Self::calculate_hash(index, timestamp, data, previous_hash, nonce);

            if hash.starts_with(&target) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str, nonce: u64)-> String {
        let mut hasher = Sha256::new();

        let input = format!(
            "{}:{}:{}:{}:{}",
            index,
            timestamp,
            data,
            previous_hash,
            nonce
        );

        hasher.update(input);

        let result = hasher.finalize(); 
        hex::encode(result)
    }
    
    fn is_valid(&self) -> bool {
        let calculated_hash = Self::calculate_hash(
            self.index,
            self.timestamp,
            &self.data,
            &self.previous_hash,
            self.nonce,
        );

        self.hash == calculated_hash
    }

    fn print(&self) {
        let validation = if self.is_valid() {
            "Valid"
        } else {
            "Invalid"
        };

        println!("Block #{} - {}", self.index, validation);
        println!("Data            : {}", self.data);
        println!("Timestamp       : {}", self.timestamp);
        println!("Previous Hash   : {}", self.previous_hash);
        println!("Hash            : {}", self.hash);
        println!("Nonce           : {}", self.nonce);
    }
}

impl Blockchain {
    fn new(genesis_block: Block, difficulty: usize) -> Self {
        Self {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, data: String) {
        let index = self.blocks.len() as u64;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let (nonce, hash) = Block::mine(index, timestamp, &data, &previous_hash, self.difficulty);

        let block = Block::new(
            index,
            timestamp,
            &data,
            &previous_hash,
            nonce,
            &hash,
        );

        self.blocks.push(block);
    }

    fn is_valid(&self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }

        let target = "0".repeat(self.difficulty);

        for i in 0..self.blocks.len() {
            let current = &self.blocks[i];

            if !current.is_valid() {
                return false;
            }

            if !current.hash.starts_with(&target) {
                return false;
            }

            if i > 0 {
                let previous = &self.blocks[i - 1];

                if current.previous_hash != previous.hash {
                    return false;
                }
            }
        }

        true
    }

    fn print(&self) {
        println!("Blockchain valid: {}", self.is_valid());
    }
}