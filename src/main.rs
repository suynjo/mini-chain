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

    Print,

    Exit,
}

fn main() {
    let difficulty: usize = 4;
    let genesis_block = Block::genesis(difficulty);
    let mut blockchain = Blockchain::new(genesis_block, difficulty);
    println!("Welcome to MINI-CHAIN CLI!");
    println!("Available commands: add --from <name> --to <name> --amount <num>, print, exit\n");

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
        // add block
        Commands::Add { ref from, ref to, amount, } => {
            let from = from;
            let to = to;
            let amount = amount;

            let transaction = Transaction::new(&from, &to, amount);
            if !transaction.is_valid() {
                println!("Invalid transaction!");
            }
            else {

            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("mining...");

            pb.enable_steady_tick(Duration::from_millis(100));
            let block = blockchain.make_block(transaction);
            println!("======================================");
            block.print();
            blockchain.add_block(block);
            println!("======================================");

            pb.finish_with_message(format!("block added!"));
            println!("Available commands: add --from <name> --to <name> --amount <num>, print, exit\n");
        }
        }

        // print all blockchain
        Commands::Print => {
            println!("======================================");
            blockchain.print();
            println!("======================================");
            println!("Available commands: add --from <name> --to <name> --amount <num>, print, exit\n");
        }

        Commands::Exit => {
            println!("Goodbye!");
            break;
        }
    }
}

}

