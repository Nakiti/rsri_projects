-- Your SQL goes here
CREATE TABLE papers (
    paperId SERIAL PRIMARY KEY,
    author INTEGER NOT NULL REFERENCES users(userId),
    title VARCHAR NOT NULL,
    abstract VARCHAR NOT NULL,
    accepted BOOLEAN NOT NULL
);