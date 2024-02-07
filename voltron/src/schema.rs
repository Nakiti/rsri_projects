// @generated automatically by Diesel CLI.

diesel::table! {
    classes (class_id) {
        class_id -> Int4,
        institution -> Varchar,
        name -> Varchar,
        instructor -> Varchar,
        editor_language -> Varchar,
        user_id -> Varchar,
    }
}

diesel::table! {
    enrollments (enrollment_id) {
        enrollment_id -> Int4,
        user_id -> Varchar,
        class_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    groups (group_id) {
        group_id -> Int4,
        name -> Varchar,
        editor_link -> Varchar,
        class_id -> Int4,
    }
}

diesel::table! {
    password_resets (password_reset_id) {
        password_reset_id -> Int4,
        email -> Varchar,
        code -> Varchar,
        valid -> Bool,
        unique_request -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
        email_address -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        theme -> Varchar,
        key_binds -> Varchar,
        admin -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(classes -> users (user_id));
diesel::joinable!(enrollments -> classes (class_id));
diesel::joinable!(enrollments -> groups (group_id));
diesel::joinable!(enrollments -> users (user_id));
diesel::joinable!(groups -> classes (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    enrollments,
    groups,
    password_resets,
    users,
);
