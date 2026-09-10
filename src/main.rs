use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

fn main() {
    let mut blockchain = Blockchain::new();
    let blocks = &blockchain.blocks;

    let index: u64 = blocks.len().try_into().unwrap(); 

    let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .unwrap();
    let data = input.trim();

    let previous_hash: String = blockchain.latest_block_hash();

    let block = Block::new(index, &data, &previous_hash);

    blockchain.add_block(block);

    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}
#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    fn new(index: u64, data : &str, previous_hash: &str)-> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let hash = Block::calculate_hash(index, timestamp, data, previous_hash);

        Block {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();

        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(&data);
        hasher.update(&previous_hash);

        let result = hasher.finalize(); 
        hex::encode(result)
    }
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "Genesis Block",
            "0",
        );

        Blockchain{
            blocks: vec![genesis_block],
        }
       
    }
    
    fn latest_block_hash(&self)->String {
        self.blocks.last().unwrap().hash.to_string()
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}