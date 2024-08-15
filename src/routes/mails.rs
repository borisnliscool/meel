use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::{database, templating, utils};
use crate::database::models::{Mail, NewMail};

#[derive(Deserialize)]
pub struct SendMailRequest {
    recipient: String,
    sender: String,
    template: String,
    priority: i32,
    data: HashMap<String, String>,
    allow_html: Option<bool>,
    schedule_at: Option<String>,
    reply_to: Option<String>,
    subject: Option<String>,
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
            scheduled_at: utils::time::system_time_to_iso_string(mail.scheduled_at),
            sent_at: mail.sent_at.map(utils::time::system_time_to_iso_string),
            sent: mail.sent_at.is_some(),
        }
    }
}

async fn send_mail(pool: Extension<Arc<database::ConnectionPool>>, mail: SendMailRequest) -> Result<Mail, StatusCode> {
    use crate::database::schema::mails;

    let html_body_string = match templating::render(mail.template.clone(), mail.data.clone(), mail.allow_html.unwrap_or(false)) {
        Ok(html_body_string) => html_body_string,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let plain_text_string = templating::render_plain_text(mail.template, mail.data).unwrap_or_else(|_| "".to_string());

    let scheduled_at = if mail.schedule_at.is_some() {
        let iso_string = match mail.schedule_at.as_ref() {
            Some(iso_string) => iso_string,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        match utils::time::iso_string_to_system_time(iso_string) {
            Ok(scheduled_at) => scheduled_at,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    } else {
        SystemTime::now()
    };

    let new_mail = NewMail {
        sender: &mail.sender,
        recipient: &mail.recipient,
        // TODO: If the subject is not passed, we should parse it from the template's metadata.
        subject: &mail.subject.unwrap_or("".to_string()),
        html_body: &html_body_string,
        text_body: &plain_text_string,
        send_attempts: 0,
        priority: mail.priority,
        reply_to: mail.reply_to.as_deref(),
        scheduled_at,
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match diesel::insert_into(mails::table)
        .values(&new_mail)
        .returning(Mail::as_returning())
        .get_result(&mut conn) {
        Ok(created_mail) => Ok(created_mail),
        Err(err) => {
            tracing::error!("{}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn send_mails(pool: Extension<Arc<database::ConnectionPool>>, Json(payload): Json<Vec<SendMailRequest>>) -> Result<Json<Vec<SendMailResponse>>, StatusCode> {
    let mut mails: Vec<Mail> = vec![];

    for mail_payload in payload {
        let created_mail = match send_mail(pool.clone(), mail_payload).await {
            Ok(created_mail) => created_mail,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        mails.push(created_mail);
    }

    Ok(Json(
        mails
            .into_iter()
            .map(SendMailResponse::new)
            .collect()
    ))
}

pub async fn get_mail_status(pool: Extension<Arc<database::ConnectionPool>>, Path(mail_id): Path<i32>) -> Result<Json<SendMailResponse>, StatusCode> {
    use crate::database::schema::mails;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mail = match mails::table
        .find(mail_id)
        .first::<Mail>(&mut conn) {
        Ok(mail) => mail,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(SendMailResponse::new(mail)))
}

pub async fn get_mail_body(pool: Extension<Arc<database::ConnectionPool>>, Path(mail_id): Path<i32>) -> Result<Html<String>, StatusCode> {
    use crate::database::schema::mails;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mail = match mails::table
        .find(mail_id)
        .first::<Mail>(&mut conn) {
        Ok(mail) => mail,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Html(mail.html_body))
}