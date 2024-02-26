-- Your SQL goes here
CREATE TABLE enrollments (
    enrollment_id SERIAL NOT NULL PRIMARY KEY,
    student_id SERIAL NOT NULL references users(user_id),
    course_id SERIAL NOT NULL references courses(course_id),
    grade VARCHAR NOT NULL
);