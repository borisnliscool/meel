use std::sync::Arc;
use std::time::SystemTime;

use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::database::ConnectionPool;
use crate::database::models::Mail;
use crate::database::schema::mails::{scheduled_at, sent_at};
use crate::database::schema::mails::dsl::mails;

async fn fetch_mails(pool: Arc<ConnectionPool>) -> Result<Vec<Mail>, StatusCode> {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut scheduled_mails = match mails
        .filter(scheduled_at.lt(SystemTime::now()))
        .filter(sent_at.is_null())
        .load::<Mail>(&mut conn) {
        Ok(scheduled_mails) => scheduled_mails,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    scheduled_mails.sort_by(|a, b| {
        b.priority.cmp(&a.priority).then_with(|| a.scheduled_at.cmp(&b.scheduled_at))
    });
    
    Ok(scheduled_mails)
}

pub async fn send_mails(pool: Arc<ConnectionPool>) {
    let scheduled_mails = match fetch_mails(pool.clone()).await {
        Ok(scheduled_mails) => scheduled_mails,
        Err(_) => return
    };

    for mail in scheduled_mails {
        tracing::info!("Sending mail {}", mail.id);
    }
}