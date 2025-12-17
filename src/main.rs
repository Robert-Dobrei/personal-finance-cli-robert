mod cli;
mod db;

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
        Commands::Import { path } => println!("Import (stub) path={path}"),
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
