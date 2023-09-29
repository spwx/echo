use std::net::SocketAddr;

use thiserror::Error;
use tokio::io::copy;
use tracing::info;

#[derive(Error, Debug)]
pub enum EchoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tokio error: {0}")]
    Task(#[from] tokio::task::JoinError),
}

#[tracing::instrument]
pub async fn echo(socket: SocketAddr) -> Result<(), EchoError> {
    // Setup tracing.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Use the below instead for tokio-console debugging.
    //
    // use tracing_subscriber::prelude::*;
    // let console_layer = console_subscriber::spawn();
    // tracing_subscriber::registry()
    //     .with(console_layer) // Add the console layer to the registry.
    //     .with(tracing_subscriber::fmt::layer()) // Add the fmt layer to the registry.
    //     .init();

    // Create a TCP listener which will listen for incoming connections.
    let listener = tokio::net::TcpListener::bind(socket).await?;

    loop {
        let (connection, _) = listener.accept().await?;
        tokio::spawn(async move { handle_connection(connection).await }).await??;
    }
}

#[tracing::instrument(fields(remote.socket = connection.peer_addr()?.to_string()), skip(connection))]
async fn handle_connection(mut connection: tokio::net::TcpStream) -> Result<(), EchoError> {
    info!("Connected");

    let (mut reader, mut writer) = connection.split();
    copy(&mut reader, &mut writer).await?;

    Ok(())
}
