ALTER TABLE mails
ADD COLUMN scheduled_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL;
