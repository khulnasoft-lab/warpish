use rusqlite::{Connection, Result};
use crate::error::{AppResult, AppError};

const DB_PATH: &str = "warpish_history.db";

/// A command history entry from the database.
#[derive(Debug, Clone)]
pub struct CommandHistory {
    pub id: i64,
    pub command: String,
    pub timestamp: String,
}

/// Handles the connection and operations for the command history database.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens a connection to the SQLite database and creates the necessary table if it doesn't exist.
    pub fn new() -> AppResult<Self> {
        let conn = Connection::open(DB_PATH)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS commands (
                id        INTEGER PRIMARY KEY,
                command   TEXT NOT NULL,
                timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(Database { conn })
    }

    /// Adds a new command to the history.
    pub fn add_command(&self, command: &str) -> AppResult<()> {
        if command.trim().is_empty() {
            return Ok(());
        }
        self.conn.execute(
            "INSERT INTO commands (command) VALUES (?1)",
            [command],
        )?;
        Ok(())
    }

    /// Retrieves all commands from history, newest first.
    pub fn get_all_commands(&self) -> AppResult<Vec<CommandHistory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, command, timestamp FROM commands ORDER BY id DESC"
        )?;
        let command_iter = stmt.query_map([], |row| {
            Ok(CommandHistory {
                id: row.get(0)?,
                command: row.get(1)?,
                timestamp: row.get(2)?,
            })
        })?;

        let mut commands = Vec::new();
        for command in command_iter {
            commands.push(command.map_err(AppError::Sqlite)?);
        }
        Ok(commands)
    }
}