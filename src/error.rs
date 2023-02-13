use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    DbError(String),
    ConfigError(String),
    UnspecifiedError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::DbError(err) => write!(f, "Database error: {}", err),
            Error::ConfigError(err) => write!(f, "Config error: {}", err),
            Error::UnspecifiedError(err) => write!(f, "Unspecified error: {}", err),
        }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::UnspecifiedError(err.parse().unwrap())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ConfigError(err.to_string())
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::DbError(err.to_string())
    }
}
