-- Your SQL goes here
CREATE TABLE users (
    user_id VARCHAR NOT NULL PRIMARY KEY,
    email_address VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    theme VARCHAR NOT NULL,
    key_binds VARCHAR NOT NULL,
    admin VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

