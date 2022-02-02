use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:8000").expect("Failed to bind to port.");
    let server = rust_ses::run(listener);
    server?.await
    // Ok(())
}
