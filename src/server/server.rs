use crate::types::Domain;
use async_std::io;
use async_std::net::{TcpListener, TcpStream, ToSocketAddrs};
use async_std::sync::RwLock;
use futures::future;
use std::collections::HashMap;

pub enum Protocol {
    HTTP,
    HTTPS,
}

struct Listener {
    listener: TcpListener,
    protocol: Protocol,
}

pub struct Server {
    domains: RwLock<HashMap<String, Domain>>,
    listeners: Vec<Listener>,
    running: bool,
}

impl Server {
    pub fn new() -> Server {
        let server = Server {
            domains: RwLock::new(HashMap::new()),
            running: false,
            listeners: Vec::new(),
        };
        server
    }

    pub async fn bind<T: ToSocketAddrs>(mut self, addr: T, protocol: Protocol) -> io::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        let listener = Listener { listener, protocol };
        self.listeners.push(listener);
        Ok(())
    }

    pub async fn add_domain(self: &Self, domain: Domain) {
        let mut map = self.domains.write().await;
        map.insert(domain.domain6.to_string(), domain);
    }

    pub async fn remove_domain(self: &Self, domain: String) {
        let mut map = self.domains.write().await;
        map.remove(&domain);
    }

    async fn accept_tcp(self: &Self, listener: &Listener) -> io::Result<(&Listener, TcpStream)> {
        let result = listener.listener.accept().await;
        match result {
            Ok((stream, _)) => Ok((listener, stream)),
            Err(e) => Err(e),
        }
    }

    async fn run(self: &Self) -> io::Result<()> {
        for &l in self.listeners {
            let lf = self.accept_tcp(l).await;
        }
        Ok(())
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
