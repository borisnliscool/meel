use std::collections::HashMap;

use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::routes::mails::{schedule_mail, send_mail};
use crate::templating;

async fn get_index() -> String {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "John".to_string());
    templating::render("greeting".to_string(), map).unwrap()
}


pub async fn create_server() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(get_index))
        .route("/mail/send", post(send_mail))
        .route("/mail/schedule", post(schedule_mail))
}