use tokio::net::TcpListener;

mod templating;
mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind to port 8080");

    let server = server::create_server().await;
    axum::serve(listener, server).await.expect("Failed to start server");
}
