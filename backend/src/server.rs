use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{delete, get, post};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::database::ConnectionPool;
use crate::routes::mailing_lists::{create_mailing_list, delete_mailing_list, get_mailing_lists, send_mailing_list_mails, subscribe_user, unsubscribe_user};
use crate::routes::mails::{get_mail_body, get_mail_status, send_mails};
use crate::routes::templates::{get_template_placeholders, get_templates, render_template, render_template_plain_text};

pub async fn create(shared_pool: Arc<ConnectionPool>) -> Router {
    let cors_layer = CorsLayer::permissive();

    Router::new()
        .route("/mails/send", post(send_mails))
        .route("/mails/:mail_id", get(get_mail_status))
        .route("/mails/:mail_id/body", get(get_mail_body))

        .route("/templates", get(get_templates))
        .route("/templates/:template_name/render", post(render_template))
        .route("/templates/:template_name/render/plain-text", post(render_template_plain_text))
        .route("/templates/:template_name/placeholders", get(get_template_placeholders))

        .route("/mailing-lists", get(get_mailing_lists))
        .route("/mailing-lists", post(create_mailing_list))
        .route("/mailing-lists/:mailing_list_id", delete(delete_mailing_list))
        .route("/mailing-lists/:mailing_list_id/subscribe", post(subscribe_user))
        .route("/mailing-lists/:mailing_list_id/unsubscribe", post(unsubscribe_user))
        .route("/mailing-lists/:mailing_list_id/send", post(send_mailing_list_mails))

        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(shared_pool))
}