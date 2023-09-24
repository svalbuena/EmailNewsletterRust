use email_newsletter::configuration::get_configuration;
use sqlx::{Connection, PgConnection};
use std::net::{SocketAddr, TcpListener};

fn spawn_app() -> String {
    // Binding to port 0 triggers an OS scan for an available port, this way we can run tests in parallel where each runs its own application
    let random_port_address = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener =
        TcpListener::bind(random_port_address).expect("Failed to bind to bind random port");
    let address = listener.local_addr().unwrap();

    let server = email_newsletter::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", address.port())
}

#[tokio::test]
async fn health_check_works() {
    let base_address = spawn_app();

    let response = reqwest::Client::new()
        .get(&format!("{}/health_check", base_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let base_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let mut connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions", base_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let base_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", base_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
