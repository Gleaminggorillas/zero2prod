//! main.rs

use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // remove env_logger
    // print all spans at info level or above if RUST_LOG hasn't been set
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    // 'With' method provided by SubscriberExt, exposed by Subscriber::tracing_subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber.");
    // get configuration file
    let configuration = get_configuration().expect("Failed to read configuration file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    // need to use the address variable
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
