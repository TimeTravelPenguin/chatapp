use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::debug;

use crate::protocol::frame::ErrorCode;
use crate::protocol::{ClientFrame, ServerFrame, recv_frame, send_frame, transport};
use crate::server::store::{self, DbStore};

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("protocol error: {0}")]
    Protocol(#[from] transport::ProtocolError),

    #[error("store error: {0}")]
    Store(#[from] store::StoreError),
}

pub async fn handle_client<S>(stream: S, store: DbStore) -> Result<(), ConnectionError>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut transport = transport::transport(stream);

    while let Some(frame) = recv_frame::<_, ClientFrame>(&mut transport).await? {
        debug!(?frame, "received client frame");

        match frame {
            ClientFrame::Signup {
                request_id,
                username,
                email,
                password,
            } => {
                // Later:
                // let user = store.create_user(username, email, password).await?;

                let _ = (store.clone(), username, email, password);

                send_frame(
                    &mut transport,
                    &ServerFrame::Error {
                        request_id: Some(request_id),
                        code: ErrorCode::NotImplemented,
                        message: "signup is not implemented yet".to_owned(),
                        details: None,
                    },
                )
                .await?;
            }

            ClientFrame::Login {
                request_id,
                email,
                password,
            } => {
                // Later:
                // let tokens = store.login(email, password).await?;

                let _ = (store.clone(), email, password);

                send_frame(
                    &mut transport,
                    &ServerFrame::Error {
                        request_id: Some(request_id),
                        code: ErrorCode::NotImplemented,
                        message: "login is not implemented yet".to_owned(),
                        details: None,
                    },
                )
                .await?;
            }
        }
    }

    Ok(())
}
