use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000").expect("Failed to bind to port.");
    let _ = rust_ses::run(listener).expect("Failed to bind address");
    Ok(())
}
