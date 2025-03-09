use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{net::IpAddr, path::PathBuf, time::Duration};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RaftConfig {
    pub node_id: u64,
    pub peers: Vec<u64>,
    pub election_timeout: u64,
    pub heartbeat_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: usize,
    pub reset_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub raft: RaftConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    pub rate_limit: RateLimitConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(File::with_name("config/default"))?;
        config.merge(File::with_name("config/local").required(false))?;
        config.merge(File::with_name("config/default"))?;
        config.merge(File::with_name("config/local").required(false))?;

        config.deserialize()
    }

    pub fn circuit_breaker_reset_timeout(&self) -> Duration {
        Duration::from_secs(self.circuit_breaker.reset_timeout)
    }

    pub fn raft_election_timeout(&self) -> Duration {
        Duration::from_millis(self.raft.election_timeout)
    }

    pub fn raft_heartbeat_interval(&self) -> Duration {
        Duration::from_millis(self.raft.heartbeat_interval)
    }
}