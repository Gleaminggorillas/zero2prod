//! main.rs

use secrecy::ExposeSecret;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
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
        PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
            &configuration.database.connection_string().expose_secret())
            .expect("Failed to create postgres connection pool");
    // need to use the address variable
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
