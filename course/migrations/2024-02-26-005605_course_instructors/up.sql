-- Your SQL goes here
CREATE TABLE course_instructors (
    course_instructor_id SERIAL NOT NULL PRIMARY KEY,
    course_id SERIAL NOT NULL references courses(course_id),
    instructor_id SERIAL NOT NULL references users(user_id)
);