use crate::schema::{users, papers, assignment_reviews, reviews, paper_coauthors};

use diesel::expression::ValidGrouping;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket::FromForm;

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub affiliation: String,
    pub level: String,
    pub password: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, ValidGrouping)]
#[diesel(table_name = papers)]
pub struct Paper {
    pub paperid: i32,
    pub author: i32,
    pub title: String,
    pub abstract_: String,
    pub accepted: bool
}

#[derive(Serialize, Deserialize, FromForm, Clone, Insertable, Queryable)]
#[diesel(table_name = papers)]
pub struct PaperDto {
    pub author: i32,
    pub title: String,
    pub abstract_: String,
    pub accepted: bool
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = assignment_reviews)]
pub struct AssignmentReview {
    pub assignment_review_id: i32,
    pub paperid: i32,
    pub userid: i32,
    pub assign_type: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = paper_coauthors)]
pub struct PaperCouthor {
    pub paper_coauthor_id: i32,
    pub paperid: i32,
    pub author: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = reviews)]
pub struct Review {
    pub reviewid: i32,
    pub paperid: i32,
    pub userid: i32,
    pub content: String,
    pub score: i32
}

pub struct UserSession {
    pub user_token: i32
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<UserSession, Self::Error> {
        let token = req.cookies().get("user_id").unwrap().value();

        let usr_token1 = token;
        println!("Your id: {}", usr_token1);

        if usr_token1.is_empty() {
            Outcome::Error((Status::Unauthorized, ()))
        } else {
            let session_user = UserSession {
                user_token: usr_token1.parse::<i32>().unwrap(),
            };
            Outcome::Success(session_user)
        }
    }
}