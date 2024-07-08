use std::time::SystemTime;

use diesel::prelude::*;

use crate::database::schema::mailing_lists;
use crate::database::schema::mails;

#[derive(Queryable, Selectable)]
#[diesel(table_name = mails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Mail {
    pub id: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub sender: String,
    pub recipient: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
    pub send_attempts: i32,
    pub priority: i32,
    pub sent_at: Option<SystemTime>,
    pub scheduled_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = mails)]
pub struct NewMail<'a> {
    pub sender: &'a str,
    pub recipient: &'a str,
    pub subject: &'a str,
    pub html_body: &'a str,
    pub text_body: &'a str,
    pub send_attempts: i32,
    pub priority: i32,
    pub scheduled_at: SystemTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = mailing_lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct MailingList {
    pub id: i32,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = mailing_lists)]
pub struct NewMailingList<'a> {
    pub name: &'a str,
    pub description: &'a str,
}