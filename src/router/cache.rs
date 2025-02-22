use std::time::Duration;
use moka::future::Cache;
use crate::types::Service;

pub struct RouteCache {
    cache: Cache<String, Vec<Service>>,
}

impl RouteCache {
    pub fn new() -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(30))
                .time_to_idle(Duration::from_secs(10))
                .max_capacity(100)
                .build(),
        }
    }

    pub async fn get(&self, key: &str) -> Option<Vec<Service>> {
        self.cache.get(key).await
    }

    pub async fn insert(&self, key: String, value: Vec<Service>) {
        self.cache.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }
}

impl Default for RouteCache {
    fn default() -> Self {
        Self::new()
    }
}