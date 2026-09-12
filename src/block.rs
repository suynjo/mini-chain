use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn genesis(difficulty: usize) -> Self {
        let (nonce, hash) = Self::mine(0, 0, "Genesis Block", "0", difficulty,);
        Self::new(0, 0, "Genesis Block", "0", nonce, &hash)
    }

    pub fn new(index: u64, timestamp: u64, transactions: &str, previous_hash: &str, nonce: u64, hash: &str)-> Self {  
        Self {
            index,
            timestamp,
            transactions: transactions.to_string(),
            previous_hash: previous_hash.to_string(),
            nonce,
            hash: hash.to_string(),
        }
    }
    
    pub fn mine(index: u64, timestamp: u64, transactions: &str, previous_hash: &str, difficulty: usize)-> (u64, String){
        let mut nonce = 0;
        let target = "0".repeat(difficulty);

        loop {
            let hash = Self::calculate_hash(index, timestamp, transactions, previous_hash, nonce);

            if hash.starts_with(&target) {
                return (nonce, hash);
            }
            nonce += 1;
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u64, transactions: &str, previous_hash: &str, nonce: u64)-> String {
        let mut hasher = Sha256::new();

        let input = format!(
            "{}:{}:{}:{}:{}",
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce
        );

        hasher.update(input);

        let result = hasher.finalize(); 
        hex::encode(result)
    }
    
    pub fn is_valid(&self) -> bool {
        let calculated_hash = Self::calculate_hash(
            self.index,
            self.timestamp,
            &self.transactions,
            &self.previous_hash,
            self.nonce,
        );

        self.hash == calculated_hash
    }

    pub fn print(&self) {
        let validation = if self.is_valid() {
            "Valid"
        } else {
            "Invalid"
        };

        println!("Block #{} - {}", self.index, validation);
        println!("Transactions    : {}", self.transactions);
        println!("Timestamp       : {}", self.timestamp);
        println!("Previous Hash   : {}", self.previous_hash);
        println!("Hash            : {}", self.hash);
        println!("Nonce           : {}", self.nonce);
    }
}
