use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

use crate::routes::mails::{get_mail_status, send_mails};
use crate::routes::templates::{get_templates, render_template};

pub async fn create() -> Router {
    Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/mails/send", post(send_mails))
        .route("/mails/:mail_id", get(get_mail_status))
        .route("/templates", get(get_templates))
        .route("/templates/:template_name/render", post(render_template))
}