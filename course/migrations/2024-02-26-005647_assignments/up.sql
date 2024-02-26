-- Your SQL goes here
CREATE TABLE assignments (
    assignment_id SERIAL NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    course_id SERIAL NOT NULL references courses(course_id),
    description VARCHAR NOT NULL
);