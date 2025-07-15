use rusqlite::{Connection, Result};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn establish_connection() -> Result<Connection> {
    // Simplified - use a fixed path for the DB file
    let db_path = "./warpish_history.db";
    let conn = Connection::open(db_path)?;
    
    // Ensure the table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            id INTEGER PRIMARY KEY,
            command TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;
    
    Ok(conn)
}

pub fn create_command(
    conn: &mut Connection,
    command_text: &str,
    success: bool,
) -> Result<usize> {
    // Get current Unix timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    conn.execute(
        "INSERT INTO commands (command, timestamp) VALUES (?, ?)",
        [command_text, &now.to_string()],
    )
}

/// Queries the command history for entries starting with a given prefix.
pub fn query_history_by_prefix(
    conn: &mut Connection,
    prefix: &str,
) -> Result<Vec<String>> {
    if prefix.is_empty() {
        return Ok(Vec::new());
    }

    let mut stmt = conn.prepare(
        "SELECT DISTINCT command FROM commands WHERE command LIKE ? ORDER BY timestamp DESC LIMIT 5"
    )?;
    
    let prefix_pattern = format!("{}%", prefix);
    let rows = stmt.query_map([prefix_pattern], |row| row.get::<_, String>(0))?;
    
    let mut results = Vec::new();
    for row_result in rows {
        results.push(row_result?);
    }
    
    Ok(results)
}

pub fn get_all_history(conn: &mut Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT command FROM commands ORDER BY timestamp DESC"
    )?;
    
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    
    let mut results = Vec::new();
    for row_result in rows {
        results.push(row_result?);
    }
    
    Ok(results)
}
