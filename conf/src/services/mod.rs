extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use dotenvy::dotenv;
use rocket::http::hyper::server::conn;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{execute, get, post};
use std::env;
use rocket::http::CookieJar;
use crate::models::{AssignmentReview, AssignmentReviewDto, Paper, PaperCouthor, PaperDto, Review, User, UserSession};
use crate::schema::assignment_reviews::{self, paperid, userid};
use crate::schema::users::{self, password, username};
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


    let users = retrieve_user(current_user);
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
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    let user = retrieve_user(user_token);

    if user[0].level == "pc" {
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
    } else {

    }
}


#[get("/paper/<paper_id>/edit")]
pub fn get_paper_edit(paper_id: i32) {
    use self::schema::papers::paperid;

    let connection = &mut establish_connection_pg();

    let paper = self::schema::papers::dsl::papers
        .filter(paperid.eq(paper_id))
        .load::<Paper>(connection)
        .expect("Error loading posts");
}


#[post("/paper/<paper_id>/edit", format="form", data="<paper>")]
pub fn edit_paper(paper_id: i32, paper: Form<PaperDto>) {
    use self::schema::papers::paperid;
    use self::schema::papers::dsl::*;

    let connection = &mut establish_connection_pg();

    diesel::update(papers)
        .filter(paperid.eq(paper_id))
        .set((abstract_.eq(paper.abstract_.to_string()), title.eq(paper.title.to_string())))
        .execute(connection)
        .expect("Error updating paper");
}

#[post("/assignReview", format="form", data="<assignment>")]
pub fn create_assignment_review(assignment: Form<AssignmentReviewDto>) {
    use self::schema::users::userid;
    use self::schema::papers::paperid;
    use self::schema::assignment_reviews::dsl::*;

    let connection = &mut establish_connection_pg();

    let is_user = retrieve_user(assignment.userid);

    let is_paper = self::schema::papers::dsl::papers
        .filter(paperid.eq(assignment.paperid))
        .load::<Paper>(connection)
        .expect("Error retrieving paper");

    if !is_user.is_empty() && !is_paper.is_empty() {

    } else {
        let new_assignment = AssignmentReviewDto {
            userid: assignment.userid,
            paperid: assignment.paperid,
            assign_type: assignment.assign_type.to_string(),
        };

        diesel::insert_into(assignment_reviews)
            .values(new_assignment)
            .execute(connection)
            .expect("Error adding assignment");
    }

}

#[get("/chair/paper/<paper_id>")]
pub fn show_paper_chair(paper_id: i32, user_session: UserSession) {
    use self::schema::reviews::paperid as review_paperid;
    use self::schema::paper_coauthors::paperid as coauthor_paperid;
    use self::schema::papers::paperid;
    use self::schema::assignment_reviews::paperid as assignment_paperid;

    let connection = &mut establish_connection_pg();
    let user_token = user_session.user_token;

    let user = retrieve_user(user_token);

    if user[0].level == "chair" {
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

        let review_assignments = self::schema::assignment_reviews::dsl::assignment_reviews
            .filter(assignment_paperid.eq(paper_id))
            .load::<AssignmentReview>(connection)
            .expect("Error loading Assignments reviews");
    }
}

#[get("/profile/<user_id>")] 
pub fn get_user_profile(user_id: i32) {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user = self::schema::users::dsl::users
        .filter(userid.eq(user_id))
        .load::<User>(connection)
        .expect("Error loading user");
}


#[get("/review/<paper_id>")]
pub fn get_review(paper_id: i32) {
    use self::schema::reviews::paperid;

    let connection = &mut establish_connection_pg();

    let reviews = self::schema::reviews::dsl::reviews
        .filter(paperid.eq(paper_id))
        .load::<Review>(connection)
        .expect("Error loading reviews");
}

#[get("/index/<option>")]
pub fn get_paper_index(option: String, user_session: UserSession) {
    use self::schema::papers::accepted;
    use self::schema::papers::author;

    let connection = &mut establish_connection_pg();
    let current_user = user_session.user_token;

    if option == "all" {

        let papers = self::schema::papers::dsl::papers
            .load::<Paper>(connection)
            .expect("Error loading papers");
    } else if option == "accepted" {

        let papers = self::schema::papers::dsl::papers
            .filter(accepted.eq(true))
            .load::<Paper>(connection)
            .expect("Error loading papers");
    } else if option == "current" {

        let papers = self::schema::papers::dsl::papers
            .filter(author.eq(current_user))
            .load::<Paper>(connection)
            .expect("Error loading papers");
    }
}

// Helper Functions
pub fn retrieve_user(user_id: i32) -> Vec<User> {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user = self::schema::users::dsl::users
        .filter(userid.eq(user_id))
        .load::<User>(connection)
        .expect("Error loading user");

    return user

}