use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use http::uri::Authority;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the server
    Server {
        /// Address to bind the server to
        #[clap(short, long, default_value = "127.0.0.1:8080")]
        address: String,

        /// Path to TLS certificate file (PEM format)
        #[clap(short, long, default_value = "./certs/cert.pem", value_parser)]
        cert: PathBuf,

        /// Path to TLS private key file (PEM format)
        #[clap(short, long, default_value = "./certs/cert.key.pem", value_parser)]
        key: PathBuf,
    },
    /// Run the client
    Client {
        /// Server address to connect to
        #[clap(
            short, 
            long, 
            default_value = "localhost:8080", 
            value_parser = parse_server_authority
        )]
        server: Authority,
    },
}

fn parse_server_authority(s: &str) -> Result<Authority, String> {
    let authority: Authority = s
        .parse()
        .map_err(|e| format!("invalid server address: {e}"))?;

    if authority.as_str().contains('@') {
        return Err("server address must not contain user info".into());
    }

    if authority.port_u16().is_none() {
        return Err("server address must include a port, e.g. localhost:8080".into());
    }

    Ok(authority)
}
