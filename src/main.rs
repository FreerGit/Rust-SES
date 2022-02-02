use rust_ses::startup;
use std::net::TcpListener;

// use startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000").expect("Failed to bind to port.");
    let server = startup::run(listener);
    server?.await
    // Ok(())
}
