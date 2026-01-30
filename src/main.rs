mod cli;
mod db;
mod parser;
mod categorize;
mod budget;
mod tui;
mod reports;

use cli::{Cli, Commands, BudgetAction};
use clap::Parser;
use crate::reports::run_report;

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

            let final_category = match category {
				Some(cat) if !cat.trim().is_empty() => cat,
				_ => categorize::auto_categorize(&desc).unwrap_or("Uncategorized".to_string()),
			};

			let tx = db::Transaction {
				date,
				amount,
				category: final_category,
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
			
			budget::check_budget(&conn, &tx).unwrap();
        }

        Commands::Budget { action } => {
			let conn = db::open_db().unwrap();

			match action {
				BudgetAction::Set { category, limit } => {
					budget::set_budget(&conn, &category, limit).unwrap();
					println!("Budget set: {} : {}", category, limit);
				}
				BudgetAction::List => {
					let budgets = budget::list_budgets(&conn).unwrap();
					for (cat, lim, spent) in budgets {
						println!("{} : {} (spent {} this month)", cat, lim, spent);
					}
				}
			}
		}
        Commands::Import { path } => {
			let conn = db::open_db().expect("Failed to open DB");

			match parser::parse_csv(&path) {
				Ok(transactions) => {
					let mut count = 0;
					for mut tx in transactions {
						if tx.category.trim().is_empty() || tx.category == "Unknown" {
							if let Some(cat) = categorize::auto_categorize(&tx.description) {
								tx.category = cat;
							}
						}

						if db::insert_transaction(&conn, &tx).is_ok() {
							count += 1;
						}
						budget::check_budget(&conn, &tx).unwrap();
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
		
		Commands::Tui => {
			let conn = db::open_db().expect("Failed to open DB");
			tui::run_tui(conn).expect("TUI failed");
		}
		
		Commands::Report => {
			let conn = db::open_db().expect("Failed to open DB");
			run_report(&conn);
		}	
	}
}
