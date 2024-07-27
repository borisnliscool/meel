use std::env;

use dotenvy::dotenv;
use tokio::net::TcpListener;

mod templating;
mod server;
mod database;
mod routes;
mod utils;

async fn start_web_server() {
    let address = env::var("MEEL_HOST").unwrap_or("meel:3000".to_string());
    let listener = TcpListener::bind(address.clone()).await.expect("Failed to bind address");

    tracing::info!("Listening on {}", address);
    let server = server::create().await;
    axum::serve(listener, server).await.expect("Failed to start server");
}

async fn start_mail_scheduler() {
    // TODO
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tokio::spawn(async {
        start_web_server().await
    });
    
    tokio::spawn(async {
        start_mail_scheduler().await
    });

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
