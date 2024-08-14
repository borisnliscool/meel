// @generated automatically by Diesel CLI.

diesel::table! {
    mail_attachments (id) {
        id -> Int4,
        mail_id -> Int4,
        created_at -> Nullable<Timestamp>,
        file_name -> Text,
        file_type -> Text,
        file_size -> Int4,
        file_path -> Text,
    }
}

diesel::table! {
    mailing_list_subscribers (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        email -> Text,
        mailing_list_id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    mailing_lists (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    mails (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        sender -> Text,
        recipient -> Text,
        subject -> Text,
        html_body -> Text,
        text_body -> Text,
        send_attempts -> Int4,
        priority -> Int4,
        sent_at -> Nullable<Timestamp>,
        scheduled_at -> Timestamp,
        reply_to -> Nullable<Text>,
    }
}

diesel::joinable!(mail_attachments -> mails (mail_id));
diesel::joinable!(mailing_list_subscribers -> mailing_lists (mailing_list_id));

diesel::allow_tables_to_appear_in_same_query!(
    mail_attachments,
    mailing_list_subscribers,
    mailing_lists,
    mails,
);
