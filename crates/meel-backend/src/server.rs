use std::sync::Arc;

use axum::routing::{get, post};
use axum::{Extension, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::database::ConnectionPool;
use crate::routes::mails::{get_mail_body, get_mail_status, send_mails};
use crate::routes::templates::{get_templates, render_template, render_template_plain_text};

pub async fn create(shared_pool: Arc<ConnectionPool>) -> Router {
    let cors_layer = CorsLayer::permissive();

    Router::new()
        .route("/mails/send", post(send_mails))
        .route("/mails/{mail_id}", get(get_mail_status))
        .route("/mails/{mail_id}/body", get(get_mail_body))
        .route("/templates", get(get_templates))
        .route("/templates/{template_name}/render", post(render_template))
        .route(
            "/templates/{template_name}/render/plain-text",
            post(render_template_plain_text),
        )
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_pool))
}
