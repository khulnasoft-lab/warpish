use rusqlite::{Connection, params};

pub fn init_db() -> Connection {
    let conn = Connection::open("warpish_history.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (id INTEGER PRIMARY KEY, command TEXT, timestamp TEXT)",
        [],
    ).unwrap();
    conn
}

pub fn save_command(conn: &Connection, command: &str) {
    let ts = chrono::Utc::now().to_rfc3339();
    conn.execute("INSERT INTO history (command, timestamp) VALUES (?1, ?2)", params![command, ts]).unwrap();
}