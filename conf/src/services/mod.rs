extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::{json::Value, json, json::Json, Deserialize, Serialize};
use rocket::{execute, get, post};
use std::env;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::FromForm;
use crate::models::{self, Paper, UserSession};

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)\
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/<user_id>")]
pub fn get_user(user_id: String) {
    use self::schema::papers::author;

    let connection = &mut establish_connection_pg();

    let papers = self::schema::papers::dsl::papers
        .filter(author.eq(userid))
        .load::<Paper>(connection)
        .expect("Error getting users");
}

#[get("/paper")]
pub fn get_paper() {

}

#[post("/paper", format="form", data="<paper>")]
pub fn create_paper(paper: Form<Paper>, user_session: UserSession) {
    use self::schema::papers::dsl::*;

    let connection = &mut establish_connection_pg();
    let user_token = &user_session.user_token;

    let new_paper = PaperDto {
        author: user_token,
        title: paper.title,
        abstract_: paper.abstract_,
        accepted: false
    }

    diesel::insert_into(papers)
        .values(new_paper)
        .execute(connection)
        .expect("Error inserting into database")
}

#[get("/paper/<paper_id>")]
pub fn show_paper(paper_id: String, user_session: UserSession) {
    
}