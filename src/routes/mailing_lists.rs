use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use diesel::{RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::database;
use crate::database::models::{MailingList, MailingListSubscriber, NewMailingList, NewMailingListSubscriber};

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

#[derive(Deserialize)]
pub struct CreateMailingListRequest {
    name: String,
    description: String,
}

pub async fn create_mailing_list(pool: Extension<Arc<database::ConnectionPool>>, Json(data): Json<CreateMailingListRequest>) -> Result<Json<MailingListResponse>, StatusCode> {
    use crate::database::schema::mailing_lists;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let new_mailing_list = NewMailingList {
        name: &data.name,
        description: &data.description,
    };

    let created_mailing_list = match diesel::insert_into(mailing_lists::table)
        .values(&new_mailing_list)
        .returning(MailingList::as_returning())
        .get_result(&mut conn) {
        Ok(created_mailing_list) => created_mailing_list,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(MailingListResponse::new(created_mailing_list)))
}

#[derive(Deserialize)]
pub struct SubscribeUserRequest {
    email: String,
    name: String,
}

#[derive(Serialize)]
pub struct SubscribeUserResponse {
    id: i32,
    email: String,
    name: String,
    mailing_list_id: i32,
}

impl SubscribeUserResponse {
    fn new(mailing_list_subscriber: MailingListSubscriber) -> Self {
        Self {
            id: mailing_list_subscriber.id,
            email: mailing_list_subscriber.email,
            name: mailing_list_subscriber.name,
            mailing_list_id: mailing_list_subscriber.mailing_list_id,
        }
    }
}

pub async fn subscribe_user(pool: Extension<Arc<database::ConnectionPool>>, Path(mailing_list_id): Path<i32>, Json(data): Json<SubscribeUserRequest>) -> Result<Json<SubscribeUserResponse>, StatusCode> {
    use crate::database::schema::mailing_list_subscribers;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let new_subscriber = NewMailingListSubscriber {
        mailing_list_id,
        email: &data.email,
        name: &data.name,
    };

    let created_subscriber = match diesel::insert_into(mailing_list_subscribers::table)
        .values(&new_subscriber)
        .returning(MailingListSubscriber::as_returning())
        .get_result(&mut conn) {
        Ok(created_subscriber) => created_subscriber,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(SubscribeUserResponse::new(created_subscriber)))
}

