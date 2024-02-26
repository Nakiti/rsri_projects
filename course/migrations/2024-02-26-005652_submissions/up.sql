-- Your SQL goes here
CREATE TABLE submissions (
    submission_id SERIAL NOT NULL PRIMARY KEY,
    assignment_id SERIAL NOT NULL references assignments(assignment_id),
    author_id SERIAL NOT NULL references users(user_id),
    content VARCHAR NOT NULL,
    grade VARCHAR NOT NULL
);