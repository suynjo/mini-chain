use sha2::{Digest, Sha256};

pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub id: String,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        let id = Self::calculate_hash(from, to, amount);
        Self {
            from : from.to_string(),
            to : to.to_string(),
            amount,
            id,
        }
    }


    fn calculate_hash(from: &str, to: &str, amount: u64) -> String {
        let mut hasher = Sha256::new();

        let input = format!(
            "{}:{}:{}",
            from,
            to,
            amount,
        );

        hasher.update(input);

        let result = hasher.finalize(); 
        hex::encode(result)
    }

    pub fn is_valid(&self) -> bool {
        !self.from.is_empty() && !self.to.is_empty() && self.from != self.to && self.amount > 0
    }
}

