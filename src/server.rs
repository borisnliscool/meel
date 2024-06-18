use std::collections::HashMap;

use axum::Router;
use axum::routing::get;
use tower_http::trace::TraceLayer;

use crate::templating;

async fn hello_world() -> String {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "John".to_string());
    templating::render("greeting".to_string(), map).unwrap()
}

pub async fn create_server() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .layer(TraceLayer::new_for_http())
}