// @generated automatically by Diesel CLI.

diesel::table! {
    assignments (assignment_id) {
        assignment_id -> Int4,
        name -> Varchar,
        course_id -> Int4,
        description -> Varchar,
    }
}

diesel::table! {
    course_instructors (course_instructor_id) {
        course_instructor_id -> Int4,
        course_id -> Int4,
        instructor_id -> Int4,
    }
}

diesel::table! {
    courses (course_id) {
        course_id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    enrollments (enrollment_id) {
        enrollment_id -> Int4,
        student_id -> Int4,
        course_id -> Int4,
        grade -> Varchar,
    }
}

diesel::table! {
    submissions (submission_id) {
        submission_id -> Int4,
        assignment_id -> Int4,
        author_id -> Int4,
        content -> Varchar,
        grade -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        email -> Varchar,
        name -> Varchar,
        role -> Varchar,
    }
}

diesel::joinable!(assignments -> courses (course_id));
diesel::joinable!(course_instructors -> courses (course_id));
diesel::joinable!(course_instructors -> users (instructor_id));
diesel::joinable!(enrollments -> courses (course_id));
diesel::joinable!(enrollments -> users (student_id));
diesel::joinable!(submissions -> assignments (assignment_id));
diesel::joinable!(submissions -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    assignments,
    course_instructors,
    courses,
    enrollments,
    submissions,
    users,
);
