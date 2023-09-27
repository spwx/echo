use std::net::SocketAddr;

use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;
use tracing_subscriber::prelude::*;

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
    let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        .with(console_layer) // Add the console layer to the registry.
        .with(tracing_subscriber::fmt::layer()) // Add the fmt layer to the registry.
        .init();

    // Create a TCP listener which will listen for incoming connections.
    let listener = tokio::net::TcpListener::bind(socket).await?;

    // Create a vector to hold task handles.
    let mut handles = Vec::new();

    // Accept 5 connections.
    for _ in 0..5 {
        let (connection, _) = listener.accept().await?;
        let handle = tokio::spawn(async move { handle_connection(connection).await });
        handles.push(handle);
    }

    // Wait for all handles to finish.
    for handle in handles {
        // The first `?` unwraps the `JoinHandle`'s `Result`.
        // The second `?` unwraps the `handle_connection`'s `Result`.
        handle.await??;
    }

    Ok(())
}

#[tracing::instrument]
async fn handle_connection(mut connection: tokio::net::TcpStream) -> Result<(), EchoError> {
    info!("Connected");

    // Create a buffer to read data from the socket into.
    let mut buf = vec![0_u8; 1024];

    loop {
        let size = connection.read_to_end(&mut buf).await?;

        if size == 0 {
            info!("Connection closed by peer");
            return Ok(());
        }

        info!("{size} bytes read");
        connection.write_all(&buf).await?;
        buf.clear();
    }
}
