//! tests/health_check.rs

use std::net::TcpListener;

// 'tokio::test' is the test equiv of 'tokio::main'
// prevents having to use the #[test] attribute
//
// remember to use expand to see whats going on!
// 'cargo expand --test <test_filename>'
#[tokio::test]
async fn health_check_works() {
    // Arrange
    // use reqwest to use HTTP against app
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // find port assigned by OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
