use crate::schema::{users, classes, groups, enrollments, password_resets};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket::FromForm;

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = users)]
pub struct User {
    user_id: String,
    email_address: String,
    first_name: String,
    last_name: String,
    theme: String,
    key_binds: String,
    admin: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(belongs_to(User))]
#[diesel(table_name = classes)]
pub struct Class {
    class_id: i32,
    institution: String,
    name: String,
    instructor: String,
    editor_lang: String,
    user_id: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(belongs_to(Class))]
#[diesel(table_name = groups)]
pub struct Group {
    group_id: i32,
    name: String,
    editor_link: String,
    class_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = enrollments)]
pub struct Enrollment {
    enrollment_id: i32,
    user_id: String,
    class_id: i32,
    group_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = password_resets)]
pub struct PasswordReset {
    password_reset_id: i32,
    email: String,
    code: String,
    valid: bool,
    unique_reset: String
}