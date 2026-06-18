use std::{net::ToSocketAddrs, path::PathBuf, sync::Arc};

use thiserror::Error;
use tokio::net::TcpListener;
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self,
        crypto::aws_lc_rs,
        pki_types::{
            CertificateDer, PrivateKeyDer,
            pem::{self, PemObject},
        },
        version,
    },
};
use tracing::{error, info};

use crate::server::{connection::handle_client, store::DbStore};

pub mod connection;
pub mod models;
pub mod password;
pub mod store;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to bind server to address: {0}")]
    BindError(#[from] std::io::Error),

    #[error("DbStore error: {0}")]
    DbStoreError(#[from] store::StoreError),

    #[error("PEM parsing error: {0}")]
    PemError(#[from] pem::Error),

    #[error("TLS configuration error: {0}")]
    RustlsError(#[from] rustls::Error),

    #[error("invalid server address: {0}")]
    InvalidAddress(String),

    #[error("client connection error: {0}")]
    ConnectionError(#[from] connection::ConnectionError),
}

pub struct Server {
    store: DbStore,
    listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl Server {
    pub async fn bind(address: &str, cert: PathBuf, key: PathBuf) -> Result<Self, ServerError> {
        let store = DbStore::connect("server.db".into()).await?;
        let address = resolve_bind_address(address)?;

        let certs = CertificateDer::pem_file_iter(&cert)?.collect::<Result<Vec<_>, _>>()?;
        let key = PrivateKeyDer::from_pem_file(&key)?;

        let config =
            rustls::ServerConfig::builder_with_provider(aws_lc_rs::default_provider().into())
                .with_protocol_versions(&[&version::TLS13, &version::TLS12])?
                .with_no_client_auth()
                .with_single_cert(certs, key)?;

        let tls_acceptor = TlsAcceptor::from(Arc::new(config));
        let listener = TcpListener::bind(address).await?;

        Ok(Self {
            store,
            listener,
            acceptor: tls_acceptor,
        })
    }

    pub async fn run(self) -> Result<(), ServerError> {
        info!("Server listening on {}", self.listener.local_addr()?);

        loop {
            let (stream, peer_addr) = self.listener.accept().await?;
            let acceptor = self.acceptor.clone();
            let store = self.store.clone();

            tokio::spawn(async move {
                let result = async move {
                    let stream = acceptor.accept(stream).await?;
                    info!(%peer_addr, "accepted TLS connection");

                    handle_client(stream, store).await?;

                    Ok::<(), ServerError>(())
                }
                .await;

                match result {
                    Ok(()) => {
                        info!(%peer_addr, "connection closed");
                    }
                    Err(error) => {
                        error!(%peer_addr, %error, "connection failed");
                    }
                }
            });
        }
    }
}

pub async fn run_server(address: &str, cert: PathBuf, key: PathBuf) -> Result<(), ServerError> {
    Server::bind(address, cert, key).await?.run().await
}

fn resolve_bind_address(address: &str) -> Result<std::net::SocketAddr, ServerError> {
    address
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| ServerError::InvalidAddress(address.to_owned()))
}
