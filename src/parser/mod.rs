use crate::db::Transaction;
use csv::ReaderBuilder;
use std::error::Error;
use chrono::NaiveDate;

pub fn parse_csv(path: &str) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut transactions = Vec::new();

    for result in reader.records() {
        let record = result?;

        if record.len() < 4 {
            eprintln!("Skipping invalid row: {:?}", record);
            continue;
        }

        let raw_date = record.get(0).unwrap().trim();

        let parsed_date = match NaiveDate::parse_from_str(raw_date, "%m/%d/%Y") {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Skipping row with invalid date: {:?}", record);
                continue;
            }
        };

        let normalized_date = parsed_date.format("%m/%d/%Y").to_string();

        let amount_str = record.get(1).unwrap().trim();
        if amount_str.is_empty() {
            eprintln!("Skipping row with empty amount: {:?}", record);
            continue;
        }

        let amount = match amount_str.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Skipping row with invalid amount: {:?}", record);
                continue;
            }
        };

        transactions.push(Transaction {
            date: normalized_date,
            amount,
            category: record.get(2).unwrap().to_string(),
            description: record.get(3).unwrap().to_string(),
        });
    }

    Ok(transactions)
}
