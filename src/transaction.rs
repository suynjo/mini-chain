use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
    timestamp: u64,
    id: String,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let id = Self::calculate_hash(from, to, amount, timestamp);
        Self {
            from : from.to_string(),
            to : to.to_string(),
            amount,
            timestamp,
            id,
        }
    }


    fn calculate_hash(from: &str, to: &str, amount: u64, timestamp: u64) -> String {
        let mut hasher = Sha256::new();

        let input = format!(
            "{}:{}:{}:{}",
            from,
            to,
            amount,
            timestamp,
        );

        hasher.update(input);

        let result = hasher.finalize(); 
        hex::encode(result)
    }

    pub fn is_valid(&self) -> bool {
        !self.from.is_empty() && !self.to.is_empty() && self.from != self.to && self.amount > 0
        && self.id == Self::calculate_hash(&self.from, &self.to, self.amount, self.timestamp)
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "- Transaction(from: {}, to: {}, amount: {}, timestamp: {}, id: {})",
            self.from,
            self.to,
            self.amount,
            self.timestamp,
            self.id
        )
    }
}