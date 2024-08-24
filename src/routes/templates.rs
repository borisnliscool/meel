use std::collections::HashMap;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::Html;
use serde::{Deserialize, Serialize};

use crate::templating;
use crate::utils::api_error::{ApiError, ApiErrorCode};

#[derive(Serialize)]
pub struct Template {
    name: String,
}

pub async fn get_templates() -> Result<Json<Vec<Template>>, ApiError> {
    let entries = match glob::glob(&format!("{}/**/*.meel", templating::get_template_directory())) {
        Ok(entries) => entries,
        Err(_) => return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to glob templates".to_string(), HashMap::new())),
    };

    let mut templates = Vec::new();
    for entry in entries {
        match entry {
            Ok(path) => {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                templates.push(Template { name });
            }
            Err(_) => {
                return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to glob templates".to_string(), HashMap::new()));
            }
        }
    }

    Ok(Json(templates))
}

#[derive(Deserialize)]
pub struct RenderTemplateRequest {
    data: HashMap<String, String>,
    allow_html: Option<bool>,
}

pub async fn render_template(Path(template_name): Path<String>, Json(data): Json<RenderTemplateRequest>) -> Result<Html<String>, ApiError> {
    match templating::render(template_name, data.data, data.allow_html.unwrap_or(false)) {
        Ok(html) => Ok(Html(html)),
        Err(err) => {
            tracing::error!("{}", err);
            Err(
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorCode::Unknown,
                    "Could not render template: ".to_string() + &err.to_string(),
                    HashMap::new(),
                )
            )
        }
    }
}

pub async fn render_template_plain_text(Path(template_name): Path<String>, Json(data): Json<HashMap<String, String>>) -> Result<String, ApiError> {
    match templating::render_plain_text(template_name, data) {
        Ok(html) => Ok(html),
        Err(err) => {
            tracing::error!("{}", err);
            Err(
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorCode::Unknown,
                    "Could not render template: ".to_string() + &err.to_string(),
                    HashMap::new(),
                )
            )
        }
    }
}

pub async fn get_template_placeholders(Path(template_name): Path<String>) -> Result<Json<Vec<String>>, ApiError> {
    match templating::get_template_placeholders(template_name) {
        Ok(vars) => Ok(Json(vars)),
        Err(err) => {
            tracing::error!("{}", err);
            Err(
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorCode::Unknown,
                    "Could not render template: ".to_string() + &err.to_string(),
                    HashMap::new(),
                )
            )
        }
    }
}