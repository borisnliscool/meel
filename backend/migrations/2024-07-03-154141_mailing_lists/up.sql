CREATE TABLE mailing_lists (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE mailing_list_subscribers (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    email TEXT NOT NULL,
    mailing_list_id INTEGER NOT NULL,

    UNIQUE(email, mailing_list_id),
    FOREIGN KEY (mailing_list_id) REFERENCES mailing_lists(id)
);