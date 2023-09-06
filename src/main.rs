use std::net::{SocketAddr, TcpListener};

use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = SocketAddr::from(([0, 0, 0, 0], 4000));
    let listener = TcpListener::bind(address).expect("Failed to bind to address");

    run(listener)?.await
}
