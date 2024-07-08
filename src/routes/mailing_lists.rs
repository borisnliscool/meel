use std::sync::Arc;

use axum::{Extension, Json};
use axum::http::StatusCode;
use diesel::RunQueryDsl;
use serde::Serialize;

use crate::database;
use crate::database::models::MailingList;

#[derive(Serialize)]
pub struct MailingListResponse {
    id: i32,
    name: String,
    description: String,
}

impl MailingListResponse {
    fn new(mailing_list: MailingList) -> Self {
        Self {
            id: mailing_list.id,
            name: mailing_list.name,
            description: mailing_list.description,
        }
    }
}

pub async fn get_mailing_lists(pool: Extension<Arc<database::ConnectionPool>>) -> Result<Json<Vec<MailingListResponse>>, StatusCode> {
    use crate::database::schema::mailing_lists;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mailing_lists = match mailing_lists::table.load::<MailingList>(&mut conn) {
        Ok(mailing_lists) => mailing_lists,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(mailing_lists.into_iter().map(MailingListResponse::new).collect()))
}