use crate::error::TlsConfigurationError;
use crate::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tokio_rustls::rustls::{Certificate, PrivateKey, ServerConfig as RustlsServerConfig};
use tokio_rustls::TlsAcceptor;

pub struct TlsConfig {
    acceptor: TlsAcceptor,
}

impl TlsConfig {
    pub fn new(cert_path: impl AsRef<Path>, key_path: impl AsRef<Path>) -> Result<Self> {
        let certs = load_certs(cert_path)?;
        let key = load_private_key(key_path)?;

        let config = RustlsServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        Ok(Self {
            acceptor: TlsAcceptor::from(Arc::new(config)),
        })
    }

    pub fn get_acceptor(&self) -> TlsAcceptor {
        self.acceptor.clone()
    }
}

fn load_certs(path: impl AsRef<Path>) -> Result<Vec<Certificate>> {
    let cert_file = std::fs::read(path)?;

    let certs = rustls_pemfile::certs(&mut &cert_file[..])?
        .into_iter()
        .map(Certificate)
        .collect();

    Ok(certs)
}

fn load_private_key(path: impl AsRef<Path>) -> Result<PrivateKey> {
    let key_file = std::fs::read(path.as_ref())?;

    let key = rustls_pemfile::pkcs8_private_keys(&mut &key_file[..])?
        .into_iter()
        .next()
        .ok_or_else(|| TlsConfigurationError::PrivateKeyNotFound(path.as_ref().to_path_buf()))?;

    Ok(PrivateKey(key))
}
