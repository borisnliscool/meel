use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::routes::mails::{get_mail_body, get_mail_status, send_mails};
use crate::routes::templates::{get_templates, render_template, render_template_plain_text, get_template_vars};

pub async fn create() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/mails/send", post(send_mails))
        .route("/mails/:mail_id", get(get_mail_status))
        .route("/mails/:mail_id/body", get(get_mail_body))
        .route("/templates", get(get_templates))
        .route("/templates/:template_name/render", post(render_template))
        .route("/templates/:template_name/render/plain-text", post(render_template_plain_text))
        .route("/templates/:template_name/vars", get(get_template_vars))
}