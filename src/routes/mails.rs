use std::collections::HashMap;
use std::time::SystemTime;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
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
    schedule_at: Option<String>,
    // TODO: attachments
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
    // TODO: attachment information
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

pub async fn send_mail(Json(payload): Json<SendMailRequest>) -> Result<Json<SendMailResponse>, StatusCode> {
    use crate::database::schema::mails;

    let html_body_string = templating::render(payload.template, payload.data).unwrap();

    let scheduled_at = if payload.schedule_at.is_some() {
        let iso_string = match payload.schedule_at.as_ref() {
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
        sender: &payload.sender,
        recipient: &payload.recipient,
        subject: "", // TODO: parse from template
        html_body: &html_body_string,
        text_body: "", // TODO: parse text body from template
        send_attempts: 0,
        priority: payload.priority,
        scheduled_at,
    };

    let mut conn = database::establish_connection();

    let created_mail = match diesel::insert_into(mails::table)
        .values(&new_mail)
        .returning(Mail::as_returning())
        .get_result(&mut conn) {
        Ok(created_mail) => created_mail,
        Err(err) => {
            tracing::error!("{}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(SendMailResponse::new(created_mail)))
}

pub async fn get_mail_status(Path(mail_id): Path<i32>) -> Result<Json<SendMailResponse>, StatusCode> {
    use crate::database::schema::mails;

    let mut conn = database::establish_connection();

    let mail = match mails::table
        .find(mail_id)
        .first::<Mail>(&mut conn) {
        Ok(mail) => mail,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    Ok(Json(SendMailResponse::new(mail)))
}