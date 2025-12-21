mod cli;
mod db;
mod parser;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            amount,
            category,
            date,
            desc,
        } => {
            let conn = db::open_db().expect("Failed to open DB");

            let tx = db::Transaction {
                date,
                amount,
                category,
                description: desc,
            };

            db::insert_transaction(&conn, &tx).expect("Failed to insert transaction");

            println!(
                "{} recorded: {} {} ({})",
                if amount < 0.0 { "Expense" } else { "Income" },
                amount.abs(),
                tx.category,
                tx.date
            );
        }

        Commands::Report => println!("Report (stub)"),
        Commands::Budget => println!("Budget (stub)"),
        Commands::Import { path } => {
			let conn = db::open_db().expect("Failed to open DB");

			match parser::parse_csv(&path) {
				Ok(transactions) => {
					let mut count = 0;
					for tx in transactions {
						if db::insert_transaction(&conn, &tx).is_ok() {
							count += 1;
						}
					}
					println!("Imported {} transactions from {}", count, path);
				}
				Err(e) => eprintln!("Import failed: {}", e),
			}
		}
        Commands::Search { category, month } => {
			let conn = db::open_db().expect("Failed to open DB");
			match db::search_transactions(&conn, category, month) {
				Ok(results) => {
					if results.is_empty() {
						println!("No transactions found.");
					} else {
						for tx in results {
							println!(
								"{} | {} | {} | {}",
								tx.date, tx.amount, tx.category, tx.description
							);
						}
					}
				}
				Err(e) => eprintln!("Search failed: {e}"),
			}
		}

    }
}
