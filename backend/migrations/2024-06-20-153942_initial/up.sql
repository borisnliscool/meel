CREATE TABLE mails (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    subject TEXT NOT NULL,
    html_body TEXT NOT NULL,
    text_body TEXT NOT NULL,
    send_attempts INTEGER NOT NULL DEFAULT 0,
    priority INTEGER NOT NULL DEFAULT 0,
    sent_at TIMESTAMP
);

CREATE INDEX mails_sender_idx ON mails (sender);
CREATE INDEX mails_recipient_idx ON mails (recipient);
CREATE INDEX mails_priority_idx ON mails (priority);
CREATE INDEX mails_created_at_idx ON mails (created_at);

CREATE TABLE mail_attachments (
    id SERIAL PRIMARY KEY,
    mail_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    file_name TEXT NOT NULL,
    file_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    FOREIGN KEY (mail_id) REFERENCES mails(id)
);

CREATE INDEX mail_attachments_mail_id_idx ON mail_attachments (mail_id);
