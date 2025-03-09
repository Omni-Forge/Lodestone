use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum TlsConfigurationError {
    #[error("Private key not found at path: {0}")]
    PrivateKeyNotFound(PathBuf),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Raft error: {0}")]
    Raft(#[from] raft::Error),
    #[error("Auth error: {0}")]
    Auth(String),
    #[error("Rate limit exceeded")]
    RateLimit,
    #[error("Config error: {0}")]
    Config(String),
    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),
    #[error("TLS error: {0}")]
    Rustls(#[from] rustls::Error),
    #[error("TLS configuration error: {0}")]
    TlsConfig(#[from] TlsConfigurationError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

impl From<std::net::AddrParseError> for Error {
  fn from(err: std::net::AddrParseError) -> Self {
      Error::BadRequest(err.to_string())
  }
}