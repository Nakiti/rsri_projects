-- Your SQL goes here
CREATE TABLE reviews (
    reviewId SERIAL PRIMARY KEY,
    paperId INTEGER NOT NULL references papers(paperId),
    userId INTEGER NOT NULL references users(userId),
    content VARCHAR NOT NULL,
    score INTEGER NOT NULL
);