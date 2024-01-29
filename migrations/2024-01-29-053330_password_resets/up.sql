-- Your SQL goes here
CREATE TABLE password_resets (
    password_reset_id SERIAL NOT NULL PRIMARY KEY,
    email VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    valid BOOLEAN NOT NULL,
    unique_request VARCHAR NOT NULL
);