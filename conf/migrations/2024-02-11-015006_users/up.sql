-- Your SQL goes here
CREATE TABLE users (
    userId SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    affiliation VARCHAR NOT NULL,
    level VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);