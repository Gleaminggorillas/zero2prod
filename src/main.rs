//! main.rs

use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // get configuration file
    let configuration = get_configuration().expect("Failed to read configuration file");
    // need to use the address variable
    let address = format!("127.0.0.1:{}", configuration.application_port);
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
