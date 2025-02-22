// src/types.rs

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Service not found: {0}")]
    NotFound(String),
    
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub health_check_url: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Service {
    pub fn new(name: String, address: String, port: u16) -> Self {
        let address_clone = address.clone();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            address,
            port,
            health_check_url: format!("http://{}:{}/health", address_clone, port),
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    pub message: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}