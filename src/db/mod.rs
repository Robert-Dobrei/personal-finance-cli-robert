use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Transaction {
    pub date: String,
    pub amount: f64,
    pub category: String,
    pub description: String,
}

pub fn open_db() -> Result<Connection> {
    let conn = Connection::open("finance.db")?;
    init_schema(&conn)?;
    Ok(conn)
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            description TEXT
        );
        "#,
    )?;
    Ok(())
}

pub fn insert_transaction(conn: &Connection, tx: &Transaction) -> Result<usize> {
    conn.execute(
        r#"
        INSERT INTO transactions (date, amount, category, description)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![tx.date, tx.amount, tx.category, tx.description],
    )
}

pub fn search_transactions(
    conn: &Connection,
    category: Option<String>,
    month: Option<String>,
) -> Result<Vec<Transaction>> {
    let mut results = Vec::new();

    if category.is_some() && month.is_some() {
        let mut stmt = conn.prepare(
            "SELECT date, amount, category, description
             FROM transactions
             WHERE category = ?1 AND date LIKE ?2",
        )?;
        let rows = stmt.query_map(
            params![category.unwrap(), format!("{}%", month.unwrap())],
            |row| {
                Ok(Transaction {
                    date: row.get(0)?,
                    amount: row.get(1)?,
                    category: row.get(2)?,
                    description: row.get(3)?,
                })
            },
        )?;
        for r in rows {
            results.push(r?);
        }
    } else if category.is_some() {
        let mut stmt = conn.prepare(
            "SELECT date, amount, category, description
             FROM transactions
             WHERE category = ?1",
        )?;
        let rows = stmt.query_map(params![category.unwrap()], |row| {
            Ok(Transaction {
                date: row.get(0)?,
                amount: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
        for r in rows {
            results.push(r?);
        }
    } else if month.is_some() {
        let mut stmt = conn.prepare(
            "SELECT date, amount, category, description
             FROM transactions
             WHERE date LIKE ?1",
        )?;
        let rows = stmt.query_map(params![format!("{}%", month.unwrap())], |row| {
            Ok(Transaction {
                date: row.get(0)?,
                amount: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
        for r in rows {
            results.push(r?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT date, amount, category, description FROM transactions",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Transaction {
                date: row.get(0)?,
                amount: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
        for r in rows {
            results.push(r?);
        }
    }

    Ok(results)
}
