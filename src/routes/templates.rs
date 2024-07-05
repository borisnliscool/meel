use std::collections::HashMap;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::Html;
use serde::{Deserialize, Serialize};

use crate::templating;

#[derive(Serialize)]
pub struct Template {
    name: String,
}

pub async fn get_templates() -> Result<Json<Vec<Template>>, StatusCode> {
    // TODO: Implement fetching templates
    Ok(Json(vec![]))
}

#[derive(Deserialize)]
pub struct RenderTemplateRequest {
    data: HashMap<String, String>,
    allow_html: Option<bool>
}

pub async fn render_template(Path(template_name): Path<String>, Json(data): Json<RenderTemplateRequest>) -> Result<Html<String>, StatusCode> {
    match templating::render(template_name, data.data, data.allow_html.unwrap_or(false)) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            tracing::error!("{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR) 
        },
    }
}

pub async fn render_template_plain_text(Path(template_name): Path<String>, Json(data): Json<HashMap<String, String>>) -> Result<String, StatusCode> {
    match templating::render_plain_text(template_name, data) {
        Ok(html) => Ok(html),
        Err(err) => {
            tracing::error!("{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR) 
        },
    }
}

pub async fn get_template_placeholders(Path(template_name): Path<String>) -> Result<Json<Vec<String>>, StatusCode> {
    match templating::get_template_placeholders(template_name) {
        Ok(vars) => Ok(Json(vars)),
        Err(err) => {
            tracing::error!("{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR) 
        },
    }
}