use std::collections::HashMap;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiErrorCode {
    Unknown,
    NotFound,
}

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub error_code: ApiErrorCode,
    pub message: String,
    pub details: HashMap<String, String>,
}

impl ApiError {
    pub fn new(status_code: StatusCode, error_code: ApiErrorCode, message: String, details: HashMap<String, String>) -> Self {
        Self {
            status_code,
            error_code,
            message,
            details,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "status_code": self.status_code.as_u16(),
            "error_code": self.error_code,
            "message": self.message,
            "details": self.details
        });

        (self.status_code, body.to_string()).into_response()
    }
}