extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{get, post};
use std::env;
use rocket::http::CookieJar;
use crate::models::{AssignmentReview, AssignmentReviewDto, NewPaper, Paper, PaperCouthor, PaperCouthorDto, PaperDto, Review, User, UserDto, UserLogin, UserSession};
use crate::schema::users::{password, username};
use crate::schema::{self};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[post("/login", format="form", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Template {
    let user_username = user.username.to_string();
    let user_password = user.password.to_string();
    let connection = &mut establish_connection_pg();

    let is_user = self::schema::users::dsl::users
        .filter((username.eq(user_username)).and(password.eq(user_password)))
        .load::<User>(connection)
        .expect("Error loading users");

    if is_user.is_empty() {
        Template::render("login", context! {})
    } else {
        let session_id = is_user[0].clone().userid.to_string();
        jar.add(("user_id", session_id));
        Template::render("profile", context! {user: is_user})
    }
}

#[get("/login")]
pub fn get_login_page() -> Template {
    Template::render("login", context! {})
}

#[post("/register", format="form", data="<user>")]
pub fn register(user: Form<UserDto>) {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let new_user = UserDto {
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

#[get("/register")] 
pub fn get_register_page() -> Template {
    Template::render("register", context! {})
}

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Template {
    jar.remove("user_id"); //removes cookies

    Template::render("login", context! {})
}

#[get("/logout")] 
pub fn get_logout_page() -> Template {
    Template::render("logout", context! {})
}


#[get("/")]
pub fn get_user(user_session: UserSession) -> Template {
    use self::schema::users::userid;

    let current_user = user_session.user_token;
    let connection = &mut establish_connection_pg();

    if is_existing_user(current_user) {
        let user = self::schema::users::dsl::users
            .filter(userid.eq(current_user))
            .load::<User>(connection)
            .expect("Error loading user");

        Template::render("profile", context! {user});
    } 

    Template::render("profile", context! {})
}

#[get("/paper")]
pub fn get_paper() -> Template {
    Template::render("paper-create", context!{}) 
}

#[post("/paper", format="form", data="<paper>")]
pub fn create_paper(paper: Form<NewPaper>, user_session: UserSession) {
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
pub fn show_paper(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::papers::paperid;
    use self::schema::reviews::paperid as review_paperid;
    use self::schema::paper_coauthors::paperid as coauthor_paperid;

    let connection = &mut establish_connection_pg();
    let user_token = user_session.user_token;
    let user = retrieve_user(user_token);

    if user[0].level == "pc" {
        let paper: Vec<Paper> = self::schema::papers::dsl::papers  
            .filter(paperid.eq(paper_id))
            .load::<Paper>(connection)
            .expect("Error retrieving paper");

        let reviews: Vec<Review> = self::schema::reviews::dsl::reviews
            .filter(review_paperid.eq(paper_id))
            .load::<Review>(connection)
            .expect("Error loading reviews");

        let authors: Vec<PaperCouthor> = self::schema::paper_coauthors::dsl::paper_coauthors
            .filter(coauthor_paperid.eq(paper_id))
            .load::<PaperCouthor>(connection)
            .expect("Error loading authors");

        Template::render("paper-show", context! {paper: &paper, authors: &authors, reviews: &reviews})
    } else {
        Template::render("paper-show", context! {})
    }
}


#[get("/paper/<paper_id>/edit")]
pub fn get_paper_edit(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::papers::paperid;

    let connection = &mut establish_connection_pg();

    let paper = self::schema::papers::dsl::papers
        .filter(paperid.eq(paper_id))
        .load::<Paper>(connection)
        .expect("Error loading posts");

    if user_session.user_token == paper[0].author {
        Template::render("paper-edit", context! {paper})
    } else {
        Template::render("paper-edit", context! {})
    }
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
pub fn create_assignment_review(assignment: Form<AssignmentReviewDto>, user_session: UserSession) -> Template {
    use self::schema::assignment_reviews::dsl::*;

    let connection = &mut establish_connection_pg();

    let new_assignment = AssignmentReviewDto {
        userid: assignment.userid,
        paperid: assignment.paperid,
        assign_type: assignment.assign_type.to_string(),
    };

    diesel::insert_into(assignment_reviews)
        .values(new_assignment)
        .execute(connection)
        .expect("Error adding assignment");

    show_paper_chair(assignment.paperid, user_session)
}

#[get("/chair/paper/<paper_id>")]
pub fn show_paper_chair(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::reviews::paperid as review_paperid;
    use self::schema::paper_coauthors::paperid as coauthor_paperid;
    use self::schema::papers::paperid;
    use self::schema::assignment_reviews::paperid as assignment_paperid;
    use self::schema::users::level;

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

        let paper_chairs = self::schema::users::dsl::users
            .filter(level.eq("pc"))
            .load::<User>(connection)
            .expect("Error loading paper chairs");


        Template::render("paper-chair", context! {paper: paper, reviews: reviews, authors: authors, review_assignments: review_assignments, paper_chairs: paper_chairs})
    } else {
        Template::render("paper-chair", context! {})
    }
}

#[get("/profile/<user_id>")] 
pub fn get_user_profile(user_id: i32) -> Template {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user = self::schema::users::dsl::users
        .filter(userid.eq(user_id))
        .load::<User>(connection)
        .expect("Error loading user");

    Template::render("profile", context! {user})
}


#[get("/review/<paper_id>")]
pub fn get_review(paper_id: i32) -> Template {
    use self::schema::reviews::paperid;

    let connection = &mut establish_connection_pg();

    let reviews = self::schema::reviews::dsl::reviews
        .filter(paperid.eq(paper_id))
        .load::<Review>(connection)
        .expect("Error loading reviews");

    Template::render("review-show", context! {reviews})
}

#[get("/review/<paper_id>/edit")]
pub fn show_edit_review(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::reviews::paperid;
    use self::schema::reviews::userid;

    let connection = &mut establish_connection_pg();

    let review = self::schema::reviews::dsl::reviews
        .filter(paperid.eq(paper_id).and(userid.eq(user_session.user_token)))
        .load::<Review>(connection)
        .expect("Error loading review");

    Template::render("paper-edit", context! {review})
}

#[post("/review/<paper_id>/edit", format="form", data="<review>")]
pub fn edit_review(paper_id: i32, user_session: UserSession, review: Form<Review> ) -> Template {
    use self::schema::reviews::reviewid;
    use self::schema::reviews::dsl::*;
    use self::schema::reviews::userid;
    use self::schema::reviews::paperid;
    use self::schema::reviews::content;
    use self::schema::reviews::score;

    let connection = &mut establish_connection_pg();

    let this_review = self::schema::reviews::dsl::reviews
        .filter((paperid.eq(paper_id)).and(userid.eq(user_session.user_token)))
        .load::<Review>(connection)
        .expect("Error retrieving review");

    if !this_review.is_empty() {
        diesel::update(reviews)
            .filter(reviewid.eq(this_review[0].reviewid))
            .set((content.eq(review.content.to_string()), (score.eq(review.score))))
            .execute(connection)
            .expect("Error updating review");
    } 
    
    Template::render("paper-edit", context!{this_review})
}

#[get("/index/<option>")]
pub fn get_paper_index(option: String, user_session: UserSession) -> Template {
    use self::schema::papers::accepted;
    use self::schema::papers::author;

    let connection = &mut establish_connection_pg();
    let current_user = user_session.user_token;

    let mut papers: Vec<Paper> = Vec::new(); 

    if option == "all" {
        papers = self::schema::papers::dsl::papers
            .load::<Paper>(connection)
            .expect("Error loading papers");

    } else if option == "accepted" {
        papers = self::schema::papers::dsl::papers
            .filter(accepted.eq(true))
            .load::<Paper>(connection)
            .expect("Error loading papers");

    } else if option == "current" {
        papers = self::schema::papers::dsl::papers
            .filter(author.eq(current_user))
            .load::<Paper>(connection)
            .expect("Error loading papers");

    }

    Template::render("paper-index", context! {papers})
}

#[post("/paperCoauthor", format="form", data="<coauthor>")]
pub fn create_paper_coauthor(coauthor: Form<PaperCouthorDto>) {
    use self::schema::paper_coauthors::dsl::*;

    let connection = &mut establish_connection_pg();

    let new_coauthor = PaperCouthorDto {
        paperid: coauthor.paperid,
        author: coauthor.author.to_string()
    };

    diesel::insert_into(paper_coauthors)
        .values(new_coauthor)
        .execute(connection)
        .expect("Error inserting coauthor");
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

pub fn is_existing_user(user_id: i32) -> bool {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user = self::schema::users::dsl::users
        .filter(userid.eq(user_id))
        .load::<User>(connection)
        .expect("Error loading user");

    if user.is_empty() {
        return false;
    } else {
        return true;
    }
}