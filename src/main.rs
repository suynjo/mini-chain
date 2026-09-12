use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;


mod block;
use crate::block::Block;

mod blockchain;
use crate::blockchain::Blockchain;

mod transaction;
use crate::transaction::Transaction;

fn main() {
    let difficulty : usize = 4;
    let genesis_block = Block::genesis(difficulty);
    let mut blockchain = Blockchain::new(genesis_block, difficulty);
    
    loop {
        println!("type your name");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let from = input.trim();

        println!("type where to send");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let to = input.trim();

        let amount = loop {
            println!("type amount");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u64>() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Please type a number!");
                }
            };
        };

        let transaction = Transaction::new(&from, &to, amount);
        if !transaction.is_valid() {
            println!("Invalid transaction!");
            continue;
        }
        
        let block = make_block(transaction, &blockchain);


        println!("======================================");
        block.print();
        blockchain.add_block(block);
        blockchain.print();
        println!("======================================");
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User(from: {}, to: {}, amount: {}, id: {})", self.from, self.to, self.amount, self.id)
    }
}

fn make_block (transaction: Transaction, blockchain: &Blockchain)-> Block {
    let index = blockchain.chain.len() as u64;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let transactions = transaction.to_string();
    let previous_hash = blockchain.chain.last().unwrap().hash.clone();
    let (nonce, hash) = Block::mine(index, timestamp, &transactions, &previous_hash, blockchain.difficulty);

    Block::new(
        index,
        timestamp,
        &transactions,
        &previous_hash,
        nonce,
        &hash,
    )
}