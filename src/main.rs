use email_newsletter::configuration::get_configuration;
use email_newsletter::startup;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = SocketAddr::from(([0, 0, 0, 0], configuration.application_port));
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    startup::run(listener)?.await
}
