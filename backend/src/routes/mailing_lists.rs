use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{database, routes};
use crate::database::models::{MailingList, MailingListSubscriber, NewMailingList, NewMailingListSubscriber};
use crate::routes::mails::SendMailRequest;
use crate::templating::{TemplateDataMap};
use crate::utils::api_error::{ApiError, ApiErrorCode};

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

pub async fn create_mailing_list(pool: Extension<Arc<database::ConnectionPool>>, Json(data): Json<CreateMailingListRequest>) -> Result<Json<MailingListResponse>, ApiError> {
    use crate::database::schema::mailing_lists;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Could not connect to database: ".to_string() + &err.to_string(), HashMap::new())
        ),
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
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to create mailing list: ".to_string() + &err.to_string(), HashMap::new())
        ),
    };

    Ok(Json(MailingListResponse::new(created_mailing_list)))
}

pub async fn delete_mailing_list(pool: Extension<Arc<database::ConnectionPool>>, Path(mailing_list_id): Path<i32>) -> Result<StatusCode, ApiError> {
    use crate::database::schema::mailing_lists;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Could not connect to database: ".to_string() + &err.to_string(), HashMap::new())),
    };

    match diesel::delete(mailing_lists::table.find(mailing_list_id)).execute(&mut conn) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::error!("{}", err);
            Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to delete mailing list".to_string(), HashMap::new()))
        }
    }
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

pub async fn subscribe_user(pool: Extension<Arc<database::ConnectionPool>>, Path(mailing_list_id): Path<i32>, Json(data): Json<SubscribeUserRequest>) -> Result<Json<SubscribeUserResponse>, ApiError> {
    use crate::database::schema::mailing_list_subscribers;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to connect to database: ".to_string() + &err.to_string(), HashMap::new())
        ),
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
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to create subscriber: ".to_string() + &err.to_string(), HashMap::new())
        ),
    };

    Ok(Json(SubscribeUserResponse::new(created_subscriber)))
}


#[derive(Deserialize)]
pub struct UnsubscribeUserRequest {
    email: String,
}

pub async fn unsubscribe_user(pool: Extension<Arc<database::ConnectionPool>>, Path(mailing_list_id): Path<i32>, Json(data): Json<UnsubscribeUserRequest>) -> Result<StatusCode, ApiError> {
    use crate::database::schema::mailing_list_subscribers;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to connect to database: ".to_string() + &err.to_string(), HashMap::new())
        ),
    };

    match diesel::delete(mailing_list_subscribers::table
        .filter(mailing_list_subscribers::mailing_list_id.eq(mailing_list_id))
        .filter(mailing_list_subscribers::email.eq(&data.email))
    ).execute(&mut conn) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::error!("{}", err);
            Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to delete subscriber: ".to_string() + &err.to_string(), HashMap::new()))
        }
    }
}

#[derive(Deserialize)]
pub struct SendMailsRequest {
    sender: String,
    template: String,
    priority: i32,
    data: TemplateDataMap,
    allow_html: Option<bool>,
    minify_html: Option<bool>,
    schedule_at: Option<String>,
    reply_to: Option<String>,
    subject: String,
    // TODO: Handle attachments
}

pub async fn send_mailing_list_mails(pool: Extension<Arc<database::ConnectionPool>>, Path(mailing_list_id): Path<i32>, Json(data): Json<SendMailsRequest>) -> Result<StatusCode, ApiError> {
    use crate::database::schema::mailing_list_subscribers;

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to connect to database: ".to_string() + &err.to_string(), HashMap::new())
        ),
    };

    let subscribers = match mailing_list_subscribers::table
        .filter(mailing_list_subscribers::mailing_list_id.eq(mailing_list_id))
        .select(MailingListSubscriber::as_select())
        .load(&mut conn) {
        Ok(subscribers) => subscribers,
        Err(err) => return Err(
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::Unknown, "Failed to load subscribers: ".to_string() + &err.to_string(), HashMap::new())
        ),
    };

    if data.subject.is_empty() || data.subject.trim().len() < 6 {
        return Err(
            ApiError::new(
                StatusCode::BAD_REQUEST,
                ApiErrorCode::Unknown,
                "Missing or invalid `subject`".to_string(),
                HashMap::new(),
            )
        );
    }

    for subscriber in subscribers {
        let mut placeholder_data = data.data.clone();
        placeholder_data.insert("subscriber_name".to_string(), Value::String(subscriber.name));
        placeholder_data.insert("subscriber_email".to_string(), Value::String(subscriber.email.clone()));

        routes::mails::send_mail(
            pool.clone(),
            SendMailRequest {
                recipient: subscriber.email,
                sender: data.sender.clone(),
                template: data.template.clone(),
                priority: data.priority,
                data: placeholder_data,
                allow_html: data.allow_html,
                minify_html: data.minify_html,
                schedule_at: data.schedule_at.clone(),
                reply_to: data.reply_to.clone(),
                subject: data.subject.clone(),
            },
        ).await?;
    }

    Ok(StatusCode::OK)
}