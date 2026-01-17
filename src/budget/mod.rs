use rusqlite::{Connection, params};
use crate::db::Transaction;
use rusqlite::OptionalExtension;
use chrono::Local;

pub fn set_budget(conn: &Connection, category: &str, limit: f64) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO budgets (category, limit_amount)
         VALUES (?1, ?2)
         ON CONFLICT(category) DO UPDATE SET limit_amount = excluded.limit_amount",
        params![category, limit],
    )?;
    Ok(())
}

pub fn list_budgets(conn: &Connection) -> rusqlite::Result<Vec<(String, f64, f64)>> {
    let mut stmt = conn.prepare("SELECT category, limit_amount FROM budgets")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
    })?;

    let now = Local::now();
    let month = now.format("%Y-%m").to_string();

    let mut result = Vec::new();

    for r in rows {
        let (category, limit) = r?;
        let spent = get_monthly_spent(conn, &category, &month)?;
        result.push((category, limit, spent));
    }

    Ok(result)
}

pub fn get_budget(conn: &Connection, category: &str) -> rusqlite::Result<Option<f64>> {
    let mut stmt = conn.prepare("SELECT limit_amount FROM budgets WHERE category = ?1")?;
    let result = stmt.query_row([category], |row| row.get(0)).optional()?;
    Ok(result)
}

pub fn get_monthly_spent(conn: &Connection, category: &str, month: &str) -> rusqlite::Result<f64> {
    let mut stmt = conn.prepare(
        "SELECT SUM(amount) FROM transactions 
        WHERE category = ?1 
		AND (substr(date, 7, 10) || '-' || substr(date, 1, 2)) = ?2"
    )?;

	let result: Option<f64> = stmt.query_row([category, month], |row| row.get::<_, Option<f64>>(0)).unwrap_or(None);
	Ok(result.unwrap_or(0.0))

}

pub fn check_budget(conn: &Connection, tx: &Transaction) -> rusqlite::Result<()> {
    let category = &tx.category;

    let Some(limit_amount) = get_budget(conn, category)? else {
        return Ok(());
    };
	
    let now = Local::now();
	let month = now.format("%Y-%m").to_string();

    let spent = get_monthly_spent(conn, category, &month)?;

    if spent < -limit_amount {
        println!(
            "ALERT: The budget for {} in month {} has been exceeded! ({} > {})",
            category, month, -spent, limit_amount
        );
    }
	
    Ok(())
}
