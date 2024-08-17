use std::sync::Arc;
use std::time::SystemTime;

use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;

use crate::database::ConnectionPool;
use crate::database::models::Mail;
use crate::database::schema::mails::{id, scheduled_at, send_attempts, sent_at};
use crate::database::schema::mails::dsl::mails;
use crate::utils;

async fn fetch_mails(pool: Arc<ConnectionPool>) -> Result<Vec<Mail>, StatusCode> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    const DEFAULT_MAX_SEND_ATTEMPTS: i32 = 10;
    let max_send_attempts: i32 = utils::env::get_var("MEEL_MAX_SEND_ATTEMPTS", Some(&DEFAULT_MAX_SEND_ATTEMPTS.to_string()))
        .ok_or(&DEFAULT_MAX_SEND_ATTEMPTS.to_string()).unwrap().parse().unwrap_or(DEFAULT_MAX_SEND_ATTEMPTS);

    let mut scheduled_mails = match mails
        .filter(scheduled_at.lt(SystemTime::now()))
        .filter(sent_at.is_null())
        .filter(send_attempts.lt(max_send_attempts))
        .load::<Mail>(&mut conn) {
        Ok(scheduled_mails) => scheduled_mails,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    scheduled_mails.sort_by(|a, b| {
        b.priority.cmp(&a.priority)
            .then_with(|| a.scheduled_at.cmp(&b.scheduled_at))
            .then_with(|| a.send_attempts.cmp(&b.send_attempts))
    });

    Ok(scheduled_mails)
}

fn get_smtp_transport() -> Result<SmtpTransport, String> {
    let smtp_username = match utils::env::get_var("MEEL_SMTP_USERNAME", None) {
        Some(username) => username,
        None => return Err("MEEL_SMTP_USERNAME must be set".to_string())
    };

    let smtp_password = match utils::env::get_var("MEEL_SMTP_PASSWORD", None) {
        Some(password) => password,
        None => return Err("MEEL_SMTP_PASSWORD must be set".to_string())
    };

    let creds = Credentials::new(smtp_username, smtp_password);
    let smtp_relay = utils::env::get_var("MEEL_SMTP_RELAY", None);

    let mailer = if smtp_relay.is_some() {
        match SmtpTransport::relay(&smtp_relay.unwrap()) {
            Ok(mailer) => mailer.credentials(creds).build(),
            Err(_) => return Err("Failed to build mailer".to_string())
        }
    } else {
        let transport_domain = utils::env::get_var("MEEL_TRANSPORT_DOMAIN", Some("mailhog"));
        let transport_port = utils::env::get_var("MEEL_TRANSPORT_PORT", Some("1025"));

        // We can use unwrap safely here, as we have a fallback set above.
        SmtpTransport::builder_dangerous(transport_domain.unwrap())
            .port(transport_port.unwrap().parse().unwrap_or(1025))
            .build()
    };

    Ok(mailer)
}

async fn send_mail(mail: Mail) -> Result<(), String> {
    let from_email: Mailbox = match mail.sender.parse() {
        Ok(email) => email,
        Err(_) => return Err("Failed to parse sender email".to_string())
    };

    let reply_to_email: Mailbox = match mail.reply_to {
        Some(reply_to) => match reply_to.parse() {
            Ok(email) => email,
            Err(_) => return Err("Failed to parse reply to email".to_string())
        },
        None => from_email.clone()
    };

    let to_email: Mailbox = match mail.recipient.parse() {
        Ok(email) => email,
        Err(_) => return Err("Failed to parse recipient email".to_string())
    };

    let email = match Message::builder()
        .from(from_email)
        .reply_to(reply_to_email)
        .to(to_email)
        .subject(mail.subject)
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(mail.text_body)
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(mail.html_body)
                )
        ) {
        Ok(email) => email,
        Err(_) => return Err("Failed to build email".to_string())
    };

    let mailer = get_smtp_transport()?;

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}

pub async fn send_mails(pool: Arc<ConnectionPool>) {
    let scheduled_mails = match fetch_mails(pool.clone()).await {
        Ok(scheduled_mails) => scheduled_mails,
        Err(_) => return
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return
    };

    for mail in scheduled_mails {
        match send_mail(mail.clone()).await {
            Ok(_) => {
                match diesel::update(mails.filter(id.eq(mail.id)))
                    .set(sent_at.eq(SystemTime::now()))
                    .execute(&mut conn) {
                    Ok(_) => tracing::info!("Sent mail {} to {}", mail.id, mail.recipient),
                    Err(_) => tracing::error!("Failed to update mail {}", mail.id)
                }
            }
            Err(err) => {
                match diesel::update(mails.filter(id.eq(mail.id)))
                    .set(send_attempts.eq(send_attempts + 1))
                    .execute(&mut conn) {
                    Ok(_) => tracing::error!("Failed to send mail {}: {}", mail.id, err),
                    Err(_) => tracing::error!("Failed to update mail {}", mail.id)
                }
            }
        }
    }
}