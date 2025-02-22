use std::sync::Arc;
use tokio_rustls::rustls::{
    Certificate, PrivateKey,
    ServerConfig as RustlsServerConfig,
};
use tokio_rustls::TlsAcceptor;

pub struct TlsConfig {
    acceptor: TlsAcceptor,
}

impl TlsConfig {
    pub fn new(cert_path: &str, key_path: &str) -> std::io::Result<Self> {
        let certs = load_certs(cert_path)?;
        let key = load_private_key(key_path)?;
        
        let config = RustlsServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
        Ok(Self {
            acceptor: TlsAcceptor::from(Arc::new(config)),
        })
    }

    pub fn get_acceptor(&self) -> TlsAcceptor {
        self.acceptor.clone()
    }
}

fn load_certs(path: &str) -> std::io::Result<Vec<Certificate>> {
    let cert_file = std::fs::read(path)?;
    let certs = rustls_pemfile::certs(&mut &cert_file[..])
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to load certificate"
        ))?
        .into_iter()
        .map(Certificate)
        .collect();
    Ok(certs)
}

fn load_private_key(path: &str) -> std::io::Result<PrivateKey> {
    let key_file = std::fs::read(path)?;
    let key = rustls_pemfile::pkcs8_private_keys(&mut &key_file[..])
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to load private key"
        ))?
        .into_iter()
        .next()
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::Other,
            "no private key found"
        ))?;
    Ok(PrivateKey(key))
}