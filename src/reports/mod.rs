use rusqlite::Connection;
use std::collections::HashMap;
use chrono::{Datelike, NaiveDate};
use crate::db::Transaction;

pub struct ReportSummary {
    pub total_income: f64,
    pub total_expenses: f64,
    pub net: f64,
    pub by_category: HashMap<String, f64>,
}

pub fn generate_monthly_summary(conn: &Connection, year: i32, month: u32) -> ReportSummary {
    let transactions = transactions_for_month(conn, year, month);

    let mut total_income = 0.0;
    let mut total_expenses = 0.0;
    let mut by_category = HashMap::new();

    for tx in transactions {
        if tx.amount < 0.0 {
            total_expenses += tx.amount.abs();
            *by_category.entry(tx.category).or_insert(0.0) += tx.amount.abs();
        } else {
            total_income += tx.amount;
        }
    }

    ReportSummary {
        total_income,
        total_expenses,
        net: total_income - total_expenses,
        by_category,
    }
}

pub fn transactions_for_month(conn: &Connection, year: i32, month: u32) -> Vec<Transaction> {
    let all = crate::db::search_transactions(conn, None, None).unwrap_or_default();

    all.into_iter()
        .filter(|tx| {
            if let Ok(date) = NaiveDate::parse_from_str(&tx.date, "%m/%d/%Y") {
                date.year() == year && date.month() == month
            } else {
                false
            }
        })
        .collect()
}

pub fn expenses_by_category_for_month(conn: &Connection, year: i32, month: u32) -> Vec<(String, f64)> {
    let txs = transactions_for_month(conn, year, month);

    let mut map = HashMap::new();

    for tx in txs {
        if tx.amount < 0.0 {
            *map.entry(tx.category).or_insert(0.0) += tx.amount.abs();
        }
    }

    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    v
}