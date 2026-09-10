use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};
use std::io;

fn main() {
    // 1. 제네시스 블럭으로 블록체인 시작.
    let mut blockchain = Blockchain::new();
    let blocks = &blockchain.blocks;

    // 2. 새 블럭 만들 준비
    // (1) index = blockchain이라는 벡터의 길이
    let index: u64 = blocks.len().try_into().unwrap(); 

    // (2) data = 입력받는 값
    let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .unwrap();
    let data = input.trim();

    // (3) previous_hash = 블록체인 벡터의 마지막 해시값
    let previous_hash: String = blockchain.latest_block_hash();

    // 3. 새 블럭 만들기
    let block = Block::new(index, &data, &previous_hash);

    // 4. 체인으로 연결하기
    // Blockchain이라는 blockchain. Blockchain이니까 그 안에 있는 add_block을 이용하였다.
    blockchain.add_block(block);

    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}
#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
}

struct Blockchain {
    blocks: Vec<Block>,
}

// u64, DateTime<Utc> -> copy
impl Block {
    fn new(index: u64, data : &str, previous_hash: &str)-> Block {
        let timestamp: DateTime<Utc> = Utc::now();
        let hash = Block::calculate_hash(index, timestamp, data, previous_hash); // 함수 호출할 때는 실제 값을 넣는다.

        Block {
            index,
            timestamp,
            data: data.to_string(),
            previous_hash: previous_hash.to_string(),
            hash,
        }
    }

    // 함수 선언할 떄는 타입도 같이.
    fn calculate_hash(index: u64, timestamp: DateTime<Utc>, data: &str, previous_hash: &str) -> String {
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
     // 첫 블럭 만들기
    fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "Genesis Block",
            "0",
        );

        // blocks 초기화
        Blockchain{
            blocks: vec![genesis_block],
        }
       
    }
    
    // 마지막 블록 가져오기
    fn latest_block_hash(&self)->String {
        self.blocks.last().unwrap().hash.to_string()
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}