use crate::database;
use crate::database::models::{Mail, NewMail};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::{Extension, Json};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use meel_templating::templating;
use meel_templating::templating::TemplateDataMap;
use meel_utils::api_error::{ApiError, ApiErrorCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

#[derive(Deserialize)]
pub struct SendMailRequest {
    pub recipient: String,
    pub sender: String,
    pub subject: String,
    pub template: String,
    pub priority: i32,
    pub data: TemplateDataMap,
    pub allow_html: Option<bool>,
    pub minify_html: Option<bool>,
    pub schedule_at: Option<String>,
    pub reply_to: Option<String>,
    // TODO: Handle attachments
}

#[derive(Serialize)]
pub struct SendMailResponse {
    id: i32,
    sender: String,
    recipient: String,
    send_attempts: i32,
    priority: i32,
    scheduled_at: String,
    sent_at: Option<String>,
    sent: bool,
    // TODO: Attachment information
}

impl SendMailResponse {
    fn new(mail: Mail) -> Self {
        Self {
            id: mail.id,
            sender: mail.sender,
            recipient: mail.recipient,
            send_attempts: mail.send_attempts,
            priority: mail.priority,
            scheduled_at: meel_utils::time::system_time_to_iso_string(mail.scheduled_at),
            sent_at: mail
                .sent_at
                .map(meel_utils::time::system_time_to_iso_string),
            sent: mail.sent_at.is_some(),
        }
    }
}

pub async fn send_mail(
    pool: Extension<Arc<database::ConnectionPool>>,
    mail: SendMailRequest,
) -> Result<Mail, ApiError> {
    use crate::database::schema::mails;

    let html_body_string = match templating::render(
        mail.template.clone(),
        mail.data.clone(),
        mail.allow_html.unwrap_or(false),
        mail.minify_html.unwrap_or(true),
    ) {
        Ok(html_body_string) => html_body_string,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::Unknown,
                "Could not render template: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };
    let plain_text_string = templating::render_plain_text(mail.template, mail.data.clone())
        .unwrap_or_else(|_| "".to_string());

    let scheduled_at = if mail.schedule_at.is_some() {
        let iso_string = match mail.schedule_at.as_ref() {
            Some(iso_string) => iso_string,
            None => {
                return Err(ApiError::new(
                    StatusCode::BAD_REQUEST,
                    ApiErrorCode::Unknown,
                    "Missing `schedule_at`".to_string(),
                    HashMap::new(),
                ))
            }
        };

        match meel_utils::time::iso_string_to_system_time(iso_string) {
            Ok(scheduled_at) => scheduled_at,
            Err(err) => {
                return Err(ApiError::new(
                    StatusCode::BAD_REQUEST,
                    ApiErrorCode::Unknown,
                    "Failed to parse `schedule_at`: ".to_string() + &err.to_string(),
                    HashMap::new(),
                ))
            }
        }
    } else {
        SystemTime::now()
    };

    // TODO: Parse the subject from the template if it is not passed by the user.

    if mail.subject.is_empty() || mail.subject.trim().len() < 6 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::Unknown,
            "Missing or invalid `subject`".to_string(),
            HashMap::new(),
        ));
    }

    let subject = templating::apply_placeholders(
        mail.subject,
        mail.data.clone(),
        // Setting allow_html to true here is a bit of a hack, as if we don't it will replace spaces
        // and special characters with html equivalents, which we don't want.
        true,
    )
    .map_err(|err| {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            ApiErrorCode::Unknown,
            "Failed to apply placeholders to subject: ".to_string() + &err.to_string(),
            HashMap::new(),
        )
    })?;

    let new_mail = NewMail {
        sender: &mail.sender,
        recipient: &mail.recipient,
        subject: &subject,
        html_body: &html_body_string,
        text_body: &plain_text_string,
        send_attempts: 0,
        priority: mail.priority,
        reply_to: mail.reply_to.as_deref(),
        scheduled_at,
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::Unknown,
                "Could not connect to database: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };

    match diesel::insert_into(mails::table)
        .values(&new_mail)
        .returning(Mail::as_returning())
        .get_result(&mut conn)
    {
        Ok(created_mail) => Ok(created_mail),
        Err(err) => {
            tracing::error!("{}", err);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::Unknown,
                "Failed to save mail: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    }
}

pub async fn send_mails(
    pool: Extension<Arc<database::ConnectionPool>>,
    Json(payload): Json<Vec<SendMailRequest>>,
) -> Result<Json<Vec<Result<SendMailResponse, ApiError>>>, ApiError> {
    let mut mails: Vec<Result<Mail, ApiError>> = vec![];

    for mail_payload in payload {
        let created_mail = send_mail(pool.clone(), mail_payload).await;
        mails.push(created_mail);
    }

    Ok(Json(
        mails
            .into_iter()
            .map(|mail| match mail {
                Ok(mail) => Ok(SendMailResponse::new(mail)),
                Err(err) => Err(err),
            })
            .collect(),
    ))
}

pub async fn get_mail_status(
    pool: Extension<Arc<database::ConnectionPool>>,
    Path(mail_id): Path<i32>,
) -> Result<Json<SendMailResponse>, ApiError> {
    use crate::database::schema::mails;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::Unknown,
                "Could not connect to database: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };

    let mail = match mails::table.find(mail_id).first::<Mail>(&mut conn) {
        Ok(mail) => mail,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::NOT_FOUND,
                ApiErrorCode::NotFound,
                "Mail not found: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };

    Ok(Json(SendMailResponse::new(mail)))
}

pub async fn get_mail_body(
    pool: Extension<Arc<database::ConnectionPool>>,
    Path(mail_id): Path<i32>,
) -> Result<Html<String>, ApiError> {
    use crate::database::schema::mails;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorCode::Unknown,
                "Could not connect to database: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };

    let mail = match mails::table.find(mail_id).first::<Mail>(&mut conn) {
        Ok(mail) => mail,
        Err(err) => {
            return Err(ApiError::new(
                StatusCode::NOT_FOUND,
                ApiErrorCode::NotFound,
                "Mail not found: ".to_string() + &err.to_string(),
                HashMap::new(),
            ))
        }
    };

    Ok(Html(mail.html_body))
}
