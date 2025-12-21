use crate::db::Transaction;
use csv::ReaderBuilder;
use std::error::Error;

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
            date: record.get(0).unwrap().to_string(),
            amount,
            category: record.get(2).unwrap().to_string(),
            description: record.get(3).unwrap().to_string(),
        });
    }

    Ok(transactions)
}
