use rand::seq::SliceRandom;
use std::sync::Arc;
use dashmap::DashMap;
use crate::service::Service;

#[derive(Debug, Clone)]
pub struct LoadBalancer {
    services: Arc<DashMap<String, Vec<Service>>>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            services: Arc::new(DashMap::new()),
        }
    }

    pub fn add_service(&self, service: Service) {
        self.services
            .entry(service.name.clone())
            .or_default()
            .push(service);
    }

    pub fn get_service(&self, name: &str) -> Option<Service> {
        self.services.get(name).and_then(|services| {
            services.choose(&mut rand::thread_rng()).cloned()
        })
    }
}