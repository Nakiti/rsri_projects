-- Your SQL goes here
CREATE TABLE users (
    user_id SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);
