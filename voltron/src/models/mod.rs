use crate::schema::{users, classes, groups, enrollments, password_resets};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket::FromForm;

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub theme: String,
    pub key_binds: String,
    pub admin: String,
    pub password: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub user_id: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub theme: String,
    pub key_binds: String,
    pub admin: String,
    pub password: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = classes)]
pub struct Class {
    pub class_id: i32,
    pub institution: String,
    pub name: String,
    pub instructor: String,
    pub editor_language: String,
    pub user_id: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(belongs_to(User))]
#[diesel(table_name = classes)]
pub struct ClassDto {
    pub institution: String,
    pub name: String,
    pub instructor: String,
    pub editor_language: String,
    pub user_id: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(belongs_to(Class))]
#[diesel(table_name = groups)]
pub struct Group {
    pub group_id: i32,
    pub name: String,
    pub editor_link: String,
    pub class_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(belongs_to(Class))]
#[diesel(table_name = groups)]
pub struct GroupDto {
    pub name: String,
    pub editor_link: String,
    pub class_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = enrollments)]
pub struct Enrollment {
    pub enrollment_id: i32,
    pub user_id: String,
    pub class_id: i32,
    pub group_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = enrollments)]
pub struct EnrollmentDto {
    pub user_id: String,
    pub class_id: i32,
    pub group_id: i32
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct EnrollmentRequestDto {
    pub class_id: i32,
    pub users: Vec<EnrollUserDto>
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct EnrollUserDto {
    pub user_id: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String, 
    pub group_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = password_resets)]
pub struct PasswordReset {
    pub password_reset_id: i32,
    pub email: String,
    pub code: String,
    pub valid: bool,
    pub unique_request: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = password_resets)]
pub struct PasswordResetDto {
    pub email: String,
    pub code: String,
    pub valid: bool,
    pub unique_request: String
}

pub struct UserSession {
    pub user_token: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<UserSession, Self::Error> {
        let token = req.cookies().get("user_id").unwrap().value();

        let usr_token1 = token.to_string();
        println!("Your id: {}", usr_token1);

        if usr_token1.is_empty() {
            Outcome::Error((Status::Unauthorized, ()))
        } else {
            let session_user = UserSession {
                user_token: usr_token1,
            };
            Outcome::Success(session_user)
        }
    }
}