//! main.rs

use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // get configuration file
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    // need to use the address variable
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}
