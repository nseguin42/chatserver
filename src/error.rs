use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Db(String),
    Configuration(String),
    Server(String),
    Unspecified(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Db(err) => write!(f, "Database error: {}", err),
            Error::Configuration(err) => write!(f, "Config error: {}", err),
            Error::Unspecified(err) => write!(f, "Unspecified error: {}", err),
            Error::Server(err) => write!(f, "Server error: {}", err),
        }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Unspecified(err.parse().unwrap())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Configuration(err.to_string())
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Error::Db(err.to_string())
    }
}

impl From<actix_web::error::Error> for Error {
    fn from(value: actix_web::Error) -> Self {
        Error::Server(value.to_string())
    }
}

impl From<Error> for actix_web::error::Error {
    fn from(value: Error) -> Self {
        actix_web::error::ErrorInternalServerError(value.to_string())
    }
}