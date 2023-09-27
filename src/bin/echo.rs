use clap::Parser;
use echo::{echo, EchoError};
use std::net::IpAddr;

#[derive(Parser)]
/// An echo server.
struct Cli {
    /// IP address to listen on.
    #[arg(default_value_t = IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)))]
    address: IpAddr,

    /// Port to listen on.
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), EchoError> {
    let cli = Cli::parse();

    let socket = std::net::SocketAddr::new(cli.address, cli.port);
    echo(socket).await
}
