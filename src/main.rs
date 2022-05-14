//! main.rs

use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // get configuration file
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to postgres");
    // need to use the address variable
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
