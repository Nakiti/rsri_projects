-- Your SQL goes here
CREATE TABLE assignment_reviews (
    assignment_review_id SERIAL PRIMARY KEY,
    paperId INTEGER NOT NULL references papers(paperId),
    userId INTEGER NOT NULL references users(userId),
    assign_type VARCHAR NOT NULL
);
