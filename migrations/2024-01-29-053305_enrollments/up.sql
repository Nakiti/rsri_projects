-- Your SQL goes here
CREATE TABLE enrollments (
    enrollment_id SERIAL NOT NULL PRIMARY KEY,
    user_id VARCHAR NOT NULL references users(user_id),
    class_id SERIAL NOT NULL references classes(class_id),
    group_id SERIAL NOT NULL references groups(group_id)
);