use std::env;
use std::sync::Arc;

use dotenvy::dotenv;
use tokio::net::TcpListener;

use crate::database::ConnectionPool;

mod templating;
mod server;
mod database;
mod routes;
mod utils;
mod mail_scheduler;

async fn start_web_server(shared_pool: Arc<ConnectionPool>) {
    let address = env::var("MEEL_HOST").unwrap_or("meel:3000".to_string());
    let listener = TcpListener::bind(address.clone()).await.expect("Failed to bind address");

    tracing::info!("Webserver listening on {}", address);
    let server = server::create(shared_pool).await;
    axum::serve(listener, server).await.expect("Failed to start server");
}

async fn start_mail_scheduler(shared_pool: Arc<ConnectionPool>) {
    tracing::info!("Starting mail scheduler");

    loop {
        mail_scheduler::send_mails(shared_pool.clone()).await;
        
        // TODO: We should make the sleep interval configurable 
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let connection_pool = database::establish_connection_pool();
    let shared_pool = Arc::new(connection_pool);

    {
        let pool = Arc::clone(&shared_pool);
        tokio::spawn(async move {
            start_web_server(pool).await
        });
    }

    {
        let pool = Arc::clone(&shared_pool);
        tokio::spawn(async move {
            start_mail_scheduler(pool).await
        });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
