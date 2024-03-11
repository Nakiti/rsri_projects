use crate::schema::{users, courses, course_instructors, enrollments, assignments, submissions};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket::FromForm;

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub name: String,
    pub role: String
}

//can remove any variables that dont need to be inputted to create a user
#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub username: String,
    pub email: String,
}

//can remove any variables that dont need to be inputted to create a user
#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub name: String,
    pub role: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = courses)]
pub struct Course {
    pub course_id: i32,
    pub name: String
}


#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = courses)]
pub struct CourseDto {
    pub name: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = course_instructors)]
pub struct CourseInstructor {
    pub course_instructor_id: i32,
    pub course_id: i32,
    pub instructor_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = course_instructors)]
pub struct CourseInstructorDto {
    pub course_id: i32,
    pub instructor_id: i32
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = enrollments)]
pub struct Enrollment {
    pub enrollment_id: i32,
    pub student_id: i32,
    pub course_id: i32,
    pub grade: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(table_name = enrollments)]
pub struct EnrollmentDto {
    pub student_id: i32,
    pub course_id: i32,
    pub grade: String
}

//make model for combining grade from enrollment + course data (students) + DELETE IF NOT NEEDED
#[derive(Serialize, Deserialize, FromForm, Queryable)]
pub struct EnrolledCourses {
    pub student_id: i32,
    pub course_id: i32,
    pub grade: String,
    pub name: String
}



#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(belongs_to(Course))]
#[diesel(table_name = assignments)]
pub struct Assignment {
    pub assignment_id: i32,
    pub name: String,
    pub course_id: i32,
    pub description: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(belongs_to(Course))]
#[diesel(table_name = assignments)]
pub struct AssignmentDto {
    pub name: String,
    pub course_id: i32,
    pub description: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(belongs_to(Course))]
#[diesel(table_name = submissions)]
pub struct Submission {
    pub submission_id: i32,
    pub assignment_id: i32,
    pub author_id: i32,
    pub content: String,
    pub grade: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone, Selectable)]
#[diesel(belongs_to(Course))]
#[diesel(table_name = submissions)]
pub struct SubmissionDto {
    pub assignment_id: i32,
    pub author_id: i32,
    pub content: String,
    pub grade: String
}

pub struct UserSession {
    pub user_token: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<UserSession, Self::Error> {
        let token = req.cookies().get("username").unwrap().value();

        //add encryption later
        let usr_token1 = token.to_string();
        println!("Your username: {}", usr_token1);

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

