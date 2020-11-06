use crate::types::Domain;
use async_std::sync::RwLock;
use std::collections::HashMap;

pub enum Protocol {
    HTTP,
    HTTPS,
}
pub struct Server {
    domains: RwLock<HashMap<String, Domain>>,
}

impl Server {
    pub fn new() -> Server {
        let server = Server {
            domains: RwLock::new(HashMap::new()),
        };
        server
    }

    pub async fn add_domain(self: &Self, domain: Domain) {
        let mut map = self.domains.write().await;
        map.insert(domain.domain6.to_string(), domain);
    }

    pub async fn remove_domain(self: &Self, domain: String) {
        let mut map = self.domains.write().await;
        map.remove(&domain);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[async_std::test]
    async fn test_new_server() {
        let domain = Domain::from_str("www.test.com".to_string());
        let server = Server::new();
        server.add_domain(domain).await;
        assert_eq!(server.domains.into_inner().len(), 1);
    }
}
