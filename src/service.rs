use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
