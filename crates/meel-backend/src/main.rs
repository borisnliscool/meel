use std::sync::Arc;

use dotenvy::dotenv;
use tokio::net::TcpListener;

use crate::database::ConnectionPool;

mod database;
mod mail_scheduler;
mod routes;
mod server;

async fn start_web_server(shared_pool: Arc<ConnectionPool>) {
    let address = meel_utils::env::get_var("MEEL_HOST", Some("0.0.0.0:8080")).unwrap();
    let listener = TcpListener::bind(address.clone())
        .await
        .expect("Failed to bind address");

    tracing::info!("Webserver listening on {}", address);
    let server = server::create(shared_pool).await;
    axum::serve(listener, server)
        .await
        .expect("Failed to start server");
}

async fn start_mail_scheduler(shared_pool: Arc<ConnectionPool>) {
    tracing::info!("Starting mail scheduler");

    loop {
        // Move this to a new thread, so it doesn't block loop interval
        tokio::spawn(mail_scheduler::send_mails(shared_pool.clone()));

        const DEFAULT_SLEEP_INTERVAL: u64 = 15;
        let sleep_interval = meel_utils::env::get_var(
            "MEEL_SCHEDULER_INTERVAL",
            Some(&DEFAULT_SLEEP_INTERVAL.to_string()),
        )
        .unwrap()
        .parse::<u64>()
        .unwrap_or(DEFAULT_SLEEP_INTERVAL);

        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_interval)).await;
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
        tokio::spawn(async move { start_web_server(pool).await });
    }

    {
        let pool = Arc::clone(&shared_pool);
        tokio::spawn(async move { start_mail_scheduler(pool).await });
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
