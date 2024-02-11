-- Your SQL goes here
CREATE TABLE paper_coauthors (
    paper_coauthor_id SERIAL PRIMARY KEY,
    paperId INTEGER NOT NULL,
    author VARCHAR NOT NULL
);