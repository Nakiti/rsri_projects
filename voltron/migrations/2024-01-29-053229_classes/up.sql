-- Your SQL goes here
CREATE TABLE classes (
    class_id SERIAL NOT NULL PRIMARY KEY,
    institution VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    instructor VARCHAR NOT NULL,
    editor_language VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL REFERENCES users(user_id)
);