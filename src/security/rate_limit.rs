use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub async fn check_rate_limit(&self, key: &str) -> bool {
        let mut requests = self.requests.lock().await;
        let now = Instant::now();
        
        // Clean up old requests
        requests.entry(key.to_string())
            .and_modify(|reqs| {
                reqs.retain(|&time| now.duration_since(time) < self.window);
            })
            .or_insert_with(Vec::new);
            
        let current_requests = requests.get(key).unwrap();
        
        if current_requests.len() >= self.max_requests {
            false
        } else {
            requests.get_mut(key).unwrap().push(now);
            true
        }
    }
}

pub fn rate_limit() -> RateLimitLayer {
    RateLimitLayer::new(
        50, // requests
        Duration::from_secs(60), // per minute
    )
}