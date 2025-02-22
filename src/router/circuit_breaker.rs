use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct CircuitBreaker {
    failure_threshold: usize,
    reset_timeout: Duration,
    failures: AtomicUsize,
    last_failure: RwLock<Option<Instant>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, reset_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            reset_timeout,
            failures: AtomicUsize::new(0),
            last_failure: RwLock::new(None),
        }
    }

    pub async fn record_success(&self) {
        self.failures.store(0, Ordering::SeqCst);
        *self.last_failure.write().await = None;
    }

    pub async fn record_failure(&self) -> bool {
        let failures = self.failures.fetch_add(1, Ordering::SeqCst) + 1;
        *self.last_failure.write().await = Some(Instant::now());
        failures >= self.failure_threshold
    }

    pub async fn is_open(&self) -> bool {
        let failures = self.failures.load(Ordering::SeqCst);
        if failures < self.failure_threshold {
            return false;
        }

        if let Some(last_failure) = *self.last_failure.read().await {
            if last_failure.elapsed() > self.reset_timeout {
                self.failures.store(0, Ordering::SeqCst);
                return false;
            }
        }

        true
    }
}
