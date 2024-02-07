-- Your SQL goes here
CREATE TABLE groups (
    group_id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    editor_link VARCHAR NOT NULL,
    class_id SERIAL NOT NULL references classes(class_id)
);