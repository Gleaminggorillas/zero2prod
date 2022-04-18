//! main.rs

use std::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if address fails to bind
    // Else, call await
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
