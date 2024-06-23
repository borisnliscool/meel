use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Template {
    name: String,
}

pub async fn get_templates() -> Result<Json<Vec<Template>>, StatusCode> {
    // TODO: Implement fetching templates
    Ok(Json(vec![]))
}