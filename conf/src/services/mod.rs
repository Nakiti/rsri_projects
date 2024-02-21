extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use dotenvy::dotenv;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{execute, get, post};
use std::env;
use rocket::http::CookieJar;
use crate::models::{Paper, PaperCouthor, PaperDto, Review, User, UserSession};
use crate::schema::users::{password, username};
use crate::schema::{self};
use rocket::form::Form;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[post("/login", format="json", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Json<User>) {

    let user_username = user.username.to_string();
    let user_password = user.password.to_string();
    let connection = &mut establish_connection_pg();

    let is_user = self::schema::users::dsl::users
        .filter((username.eq(user_username)).and(password.eq(user_password)))
        .load::<User>(connection)
        .expect("Error loading users");

    if is_user.is_empty() {

    } else {
        let session_id = is_user[0].clone().userid.to_string();
        jar.add(("user_id", session_id));
    }
}

#[post("/register", format="json", data="<user>")]
pub fn register(user: Json<User>) {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let new_user = User {
        userid:  1,
        username: user.username.to_string(),
        name: user.name.to_string(),
        email: user.email.to_string(),
        affiliation: user.affiliation.to_string(),
        level: user.level.to_string(),
        password: user.password.to_string()
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error adding user");
}

#[get("/")]
pub fn get_user(user_session: UserSession) {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();
    let current_user = user_session.user_token;


    let users = self::schema::users::dsl::users
        .filter(userid.eq(current_user))
        .load::<User>(connection)
        .expect("Error loading user");
}

#[get("/paper")]
pub fn get_paper() {

}

#[post("/paper", format="form", data="<paper>")]
pub fn create_paper(paper: Form<Paper>, user_session: UserSession) {
    use self::schema::papers::dsl::*;

    let connection = &mut establish_connection_pg();
    let current_user = user_session.user_token;

    let new_paper = PaperDto {
        author: current_user,
        title: paper.title.to_string(),
        abstract_: paper.abstract_.to_string(),
        accepted: false
    };

    diesel::insert_into(papers)
        .values(new_paper)
        .execute(connection)
        .expect("Error inserting into database");
}

#[get("/paper/<paper_id>")]
pub fn show_paper(paper_id: i32, user_session: UserSession) {
    use self::schema::papers::paperid;
    use self::schema::reviews::paperid as review_paperid;
    use self::schema::paper_coauthors::paperid as coauthor_paperid;

    let connection = &mut establish_connection_pg();

    let user_token = &user_session.user_token;

    let paper = self::schema::papers::dsl::papers  
        .filter(paperid.eq(paper_id))
        .load::<Paper>(connection)
        .expect("Error retrieving paper");

    let reviews = self::schema::reviews::dsl::reviews
        .filter(review_paperid.eq(paper_id))
        .load::<Review>(connection)
        .expect("Error loading reviews");

    let authors = self::schema::paper_coauthors::dsl::paper_coauthors
        .filter(coauthor_paperid.eq(paper_id))
        .load::<PaperCouthor>(connection)
        .expect("Error loading authors");

}