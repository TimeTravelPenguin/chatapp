use http::uri::Authority;
use thiserror::Error;
use tracing::info;

use crate::client::app::ClientApp;

mod app;
mod screens;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("failed to connect to server: {0}")]
    ConnectionError(Authority),
    #[error("iced application error: {0}")]
    IcedError(#[from] iced::Error),
}

pub async fn run_client(server: Authority) -> Result<(), ClientError> {
    info!(server = %server, "Starting client");
    iced::application(
        move || ClientApp::new(server.clone()),
        ClientApp::update,
        ClientApp::view,
    )
    .theme(|app: &ClientApp| app.theme.clone())
    .run()?;

    Ok(())
}
