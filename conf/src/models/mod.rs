use crate::schema::{users, papers, assignment_reviews, reviews, paper_coauthors};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::http::Status;
use rocket::FromForm;

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = users)]
pub struct User {
    userid: Int4,
    username: String,
    name: String,
    email: String,
    affiliation: String,
    level: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = papers)]
pub struct Paper {
    paperid: Int4,
    author: Int4,
    title: String,
    abstract_: String,
    accepted: Bool
}

#[derive(Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = papers)]
pub struct PaperDto {
    title: String,
    abstract_: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = assignment_reviews)]
pub struct AssignmentReview {
    assignment_review_id: Int4,
    paperid: Int4,
    userid: Int4,
    assign_type: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = paper_coauthors)]
pub struct PaperCouthor {
    paper_coauthor_id: Int4,
    paperid: Int4,
    author: String
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm, Clone)]
#[diesel(table_name = reviews)]
pub struct Review {
    reviewid: Int4,
    paperid: Int4,
    userid: Int4,
    content: String,
    score: Int4
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