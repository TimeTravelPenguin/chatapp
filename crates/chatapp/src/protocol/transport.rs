use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub type Transport<S> = Framed<S, LengthDelimitedCodec>;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn transport<S>(stream: S) -> Transport<S>
where
    S: AsyncRead + AsyncWrite,
{
    let codec = LengthDelimitedCodec::builder()
        .max_frame_length(1024 * 1024)
        .new_codec();

    Framed::new(stream, codec)
}

pub async fn send_frame<S, T>(transport: &mut Transport<S>, frame: &T) -> Result<(), ProtocolError>
where
    S: AsyncRead + AsyncWrite + Unpin,
    T: serde::Serialize,
{
    let bytes = serde_json::to_vec(frame)?;
    transport.send(Bytes::from(bytes)).await?;
    Ok(())
}

pub async fn recv_frame<S, T>(transport: &mut Transport<S>) -> Result<Option<T>, ProtocolError>
where
    S: AsyncRead + AsyncWrite + Unpin,
    T: for<'de> serde::Deserialize<'de>,
{
    let Some(frame) = transport.next().await else {
        return Ok(None);
    };

    let bytes = frame?;
    let value = serde_json::from_slice(&bytes)?;

    Ok(Some(value))
}
