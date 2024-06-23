use std::collections::HashMap;

use axum::http::StatusCode;
use axum::Json;
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::{database, templating};
use crate::models::{Mail, NewMail};

#[derive(Deserialize)]
pub struct SendMailRequest {
    recipient: String,
    sender: String,
    template: String,
    priority: i32,
    data: HashMap<String, String>,
    // TODO: attachments
}

#[derive(Serialize)]
pub struct SendMailResponse {
    id: i32,
    sender: String,
    recipient: String,
    priority: i32,
}

impl SendMailResponse {
    fn new(mail: Mail) -> Self {
        Self {
            id: mail.id,
            sender: mail.sender,
            recipient: mail.recipient,
            priority: mail.priority,
        }
    }
}

pub async fn send_mail(Json(payload): Json<SendMailRequest>) -> Result<Json<SendMailResponse>, StatusCode> {
    use crate::schema::mails;

    let html_body_string = templating::render(payload.template, payload.data).unwrap();

    let new_mail = NewMail {
        sender: &payload.sender,
        recipient: &payload.recipient,
        subject: "", // TODO: parse from template
        html_body: &html_body_string,
        text_body: "", // TODO: parse text body from template
        send_attempts: 0,
        priority: payload.priority,
    };

    let mut conn = database::establish_connection();

    let created_mail = match diesel::insert_into(mails::table)
        .values(&new_mail)
        .returning(Mail::as_returning())
        .get_result(&mut conn) {
        Ok(created_mail) => created_mail,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(SendMailResponse::new(created_mail)))
}

pub async fn schedule_mail() -> String {
    todo!("Schedule email")
}