use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::routes::mails::{get_mail_status, send_mail};
use crate::routes::templates::get_templates;

pub async fn create_server() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/mail/send", post(send_mail))
        .route("/mail/:mail_id", get(get_mail_status))
        .route("/templates", get(get_templates))
}