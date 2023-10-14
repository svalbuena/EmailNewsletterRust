use email_newsletter::configuration::get_configuration;
use email_newsletter::{startup, telemetry};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        telemetry::get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let address = SocketAddr::from(([0, 0, 0, 0], configuration.application_port));
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    startup::run(listener, connection_pool)?.await
}
