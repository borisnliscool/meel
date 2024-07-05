use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::database;
use crate::routes::mails::{get_mail_body, get_mail_status, send_mails};
use crate::routes::templates::{get_template_placeholders, get_templates, render_template, render_template_plain_text};

pub async fn create() -> Router {
    let pool = database::establish_connection_pool();
    let shared_pool = Arc::new(pool);

    Router::new()
        .route("/mails/send", post(send_mails))
        .route("/mails/:mail_id", get(get_mail_status))
        .route("/mails/:mail_id/body", get(get_mail_body))
        .route("/templates", get(get_templates))
        .route("/templates/:template_name/render", post(render_template))
        .route("/templates/:template_name/render/plain-text", post(render_template_plain_text))
        .route("/templates/:template_name/placeholders", get(get_template_placeholders))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_pool))
}