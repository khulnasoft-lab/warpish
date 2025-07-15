use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("PTY error: {0}")]
    Pty(String),
    #[error("Clipboard error: {0}")]
    Clipboard(String),
    #[error("Other error: {0}")]
    Other(String),
}
