use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use std::io::self; 

mod block;
use crate::block::Block;

mod blockchain;
use crate::blockchain::Blockchain;

mod transaction;
use crate::transaction::Transaction;

#[derive(Parser)]
#[command(name = "mini-chain-Cli", version = "1.0", about = "MINI-CHAIN")]


struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(long)]
        from: String,

        #[arg(long)]
        to: String,

        #[arg(long)]
        amount: u64,
    },

    Check,

    Mine,

    Print,

    Exit,
}

fn main() {
    let difficulty: usize = 4;
    let genesis_block = Block::genesis(difficulty);
    let mut blockchain = Blockchain::new(genesis_block, difficulty);

    let mut pending_transactions: Vec<Transaction> = Vec::new();

    println!("Welcome to MINI-CHAIN CLI!");
    println!("- [Add transaction] add --from <name> --to <name> --amount <num>");
    println!("- [Exit] exit");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
        args.insert(0, "mini-chain".to_string());

        let cli_args = match Cli::try_parse_from(args) {
            Ok(parsed) => parsed,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        match cli_args.command {

        Commands::Add { ref from, ref to, amount, } => {
            let transaction = Transaction::new(from, to, amount);
            if !transaction.is_valid() {
                println!("Invalid transaction!");
            }
            else {
                pending_transactions.push(transaction);
                println!("======================================");
                println!("Transaction added to the pending pool.");
                println!("- [Add transaction] add --from <name> --to <name> --amount <num>");
                println!("- [Check transactions] check");
            }
        }

        Commands::Check => {
            if pending_transactions.is_empty() {
                println!("======================================");
                println!("No pending transactions.");
            } else {
                println!("======================================");
                for tx in &pending_transactions {
                    println!("{}", tx);
                }
            }
            println!("======================================");
            println!("- [Add transaction] add --from <name> --to <name> --amount <num>");
            println!("- [Start mining] mine");
        }

        Commands::Mine => {
            if pending_transactions.is_empty() {
                println!("No pending transactions to mine.");
                continue;
            }

            let transactions = pending_transactions.iter().map(|tx| tx.to_string()).collect::<Vec<String>>().join("|");

            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Mining...");

            pb.enable_steady_tick(Duration::from_millis(100));
            let block = blockchain.make_block(transactions);
            println!("======================================");
            block.print();
            blockchain.add_block(block);
            pending_transactions.clear();
            println!("======================================");
            pb.finish_with_message("Block added!");
            println!("======================================");
            println!("- [Add transaction] add --from <name> --to <name> --amount <num>");
            println!("- [Print blockchain] print");
            println!("- [Exit] exit");
        }

        Commands::Print => {
            println!("======================================");
            blockchain.print();
            println!("======================================");
            println!("- [Add transaction] add --from <name> --to <name> --amount <num>");
            println!("- [Exit] exit");
        }

        Commands::Exit => {
            println!("Goodbye!");
            break;
        }
    }
}

}

