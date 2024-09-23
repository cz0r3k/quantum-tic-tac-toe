use crate::server_error::ServerError;
use error_stack::ResultExt;
use ipc::from_server::FromServer;
use ipc::to_server::ToServer;
use log::info;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const BUFFER_SIZE: usize = 1024;

pub async fn read_message<Reader: AsyncRead + Unpin>(
    mut reader: Reader,
) -> error_stack::Result<ToServer, ServerError> {
    let mut buf = [0; BUFFER_SIZE];
    let n = reader
        .read(&mut buf)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Error reading")?;
    info!("Read {n} bytes");
    if n == 0 {
        return Ok(ToServer::EndConnection);
    }
    let decoded = bincode::deserialize::<ToServer>(&buf[..n])
        .change_context(ServerError::SerializationError)
        .attach_printable("Can't be deserialized")?;
    Ok(decoded)
}

pub async fn write_message<Writer: AsyncWrite + Unpin>(
    mut writer: Writer,
    message: &FromServer,
) -> error_stack::Result<(), ServerError> {
    let encode: Vec<u8> = bincode::serialize(&message)
        .change_context(ServerError::SerializationError)
        .attach_printable("Can't be serialized")?;
    let n = writer
        .write(&encode)
        .await
        .change_context(ServerError::TCPError)
        .attach_printable("Error writing")?;
    info!("Write {n} bytes");
    Ok(())
}
