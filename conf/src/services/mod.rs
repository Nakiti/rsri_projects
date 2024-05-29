extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
// use diesel::{connection, prelude::*};
use dotenvy::dotenv;
use rocket::http::hyper::server::conn;
use rocket::{get, post};
use std::env;
use rocket::http::CookieJar;
use crate::models::{AssignmentReview, AssignmentReviewDto, NewPaper, Paper, PaperCouthor, PaperCouthorDto, PaperDto, Review, User, UserDto, UserLogin, UserSession};
use crate::schema::users::{password, username};
use crate::schema::{self, papers}; 
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rdiesel::{select_list, update_where, Expr, Field};
use diesel::{RunQueryDsl, Connection};



pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl rdiesel::Expr<User, String> for schema::users::username {}
impl rdiesel::Expr<User, String> for schema::users::password {}

// logs user in
#[post("/login", format="form", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Template {
    let user_username = user.username.to_string();
    let user_password = user.password.to_string();
    let connection = &mut establish_connection_pg();

    let q1 = username.eq(user_username);
    let q2 = password.eq(user_password);
    let q3 = q1.and(q2);
    let is_user = select_list(connection, q3).expect("Error retrieving user");

    // let is_user = self::schema::users::dsl::users
    //     .filter((username.eq(user_username)).and(password.eq(user_password)))
    //     .load::<User>(connection)
    //     .expect("Error loading users");

    if is_user.is_empty() {
        Template::render("login", context! {})
    } else {
        let session_id = is_user[0].clone().userid.to_string();
        jar.add(("user_id", session_id));
        Template::render("profile", context! {user: is_user})
    }
}

// displays login page
#[get("/login")]
pub fn get_login_page() -> Template {
    Template::render("login", context! {})
}

// creates new user
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

// displays register page 
#[get("/register")] 
pub fn get_register_page() -> Template {
    Template::render("register", context! {})
}

// deletes cookies and logs user out
#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Template {
    jar.remove("user_id"); //removes cookies

    Template::render("login", context! {})
}

// displays logout page
#[get("/logout")] 
pub fn get_logout_page() -> Template {
    Template::render("logout", context! {})
}

impl rdiesel::Expr<Paper, i32> for schema::papers::author {}

// displays home page
#[get("/")]
pub fn home(user_session: UserSession) -> Template {
    use self::schema::papers::author;

    let current_user = user_session.user_token;
    let connection = &mut establish_connection_pg();

    let q1 = author.eq(current_user);
    let papers = select_list(connection, q1).expect("Error retrieving papers");

    // let papers = self::schema::papers::dsl::papers
    //     .filter(author.eq(current_user))
    //     .load::<Paper>(connection)
    //     .expect("Error loading papers");

    println!("{}", papers[0].title);
    Template::render("home", context! {papers})
}

// displays create paper form
#[get("/paper")]
pub fn get_paper() -> Template {
    Template::render("paper-create", context!{}) 
}

// creates new paper
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

impl rdiesel::Expr<Paper, i32> for schema::papers::paperid {}
impl rdiesel::Expr<Review, i32> for schema::reviews::paperid  {}
impl rdiesel::Expr<PaperCouthor, i32> for schema::paper_coauthors::paperid {}
// displays paper
#[get("/paper/<paper_id>")]
pub fn show_paper(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::papers::paperid;
    use self::schema::reviews::paperid as review_paperid;
    use self::schema::paper_coauthors::paperid as coauthor_paperid;

    let connection = &mut establish_connection_pg();
    let user_token = user_session.user_token;
    let user = retrieve_user(user_token);

    if user[0].level == "pc" || user[0].level == "chair" {
        let paper_q1 = paperid.eq(paper_id);
        let paper = select_list(connection, paper_q1).expect("Error retrieving papers");
    
        let review_q1 = review_paperid.eq(paper_id);
        let reviews =  select_list(connection, review_q1).expect("Error retrieving reviews");
    
        let authors_q1 = coauthor_paperid.eq(paper_id);
        let authors = select_list(connection, authors_q1).expect("Error retrieving authors");

        // let paper: Vec<Paper> = self::schema::papers::dsl::papers  
        //     .filter(paperid.eq(paper_id))
        //     .load::<Paper>(connection)
        //     .expect("Error retrieving paper");

        // let reviews: Vec<Review> = self::schema::reviews::dsl::reviews
        //     .filter(review_paperid.eq(paper_id))
        //     .load::<Review>(connection)
        //     .expect("Error loading reviews");

        // let authors: Vec<PaperCouthor> = self::schema::paper_coauthors::dsl::paper_coauthors
        //     .filter(coauthor_paperid.eq(paper_id))
        //     .load::<PaperCouthor>(connection)
        //     .expect("Error loading authors");

        Template::render("paper-show", context! {paper: &paper, authors: &authors, reviews: &reviews})
    } else {
        Template::render("paper-show", context! {})
    }
}

// displays paper edit form
#[get("/paper/<paper_id>/edit")]
pub fn get_paper_edit(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::papers::paperid;

    let connection = &mut establish_connection_pg();

    let paper_q1 = paperid.eq(paper_id);
    let paper = select_list(connection, paper_q1).expect("Error retrieving paper");

    // let paper = self::schema::papers::dsl::papers
    //     .filter(paperid.eq(paper_id))
    //     .load::<Paper>(connection)
    //     .expect("Error loading posts");

    if user_session.user_token == paper[0].author {
        Template::render("paper-edit", context! {paper})
    } else {
        Template::render("paper-edit", context! {})
    }
}

impl rdiesel::Field<Paper, i32> for schema::papers::paperid {}

impl rdiesel::Expr<Paper, String> for schema::papers::abstract_ {}
impl rdiesel::Field<Paper, String> for schema::papers::abstract_ {}

impl rdiesel::Expr<Paper, String> for schema::papers::title {}
impl rdiesel::Field<Paper, String> for schema::papers::title {}

// edits paper
#[post("/paper/<paper_id>/edit", format="form", data="<paper>")]
pub fn edit_paper(paper_id: i32, paper: Form<PaperDto>) {
    use self::schema::papers::paperid;
    use self::schema::papers::dsl::*;

    let connection = &mut establish_connection_pg();

    let papers_q1 = paperid.eq(paper_id);
    
    let update_ = update_where(
        connection, 
        papers_q1, 
        (abstract_.assign(paper.abstract_.to_string()), title.assign(paper.title.to_string()))
    );

    // diesel::update(papers)
    //     .filter(paperid.eq(paper_id))
    //     .set((abstract_.eq(paper.abstract_.to_string()), title.eq(paper.title.to_string())))
    //     .execute(connection)
    //     .expect("Error updating paper");
}

// creates new review assignment
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

impl rdiesel::Expr<Paper, bool> for schema::papers::accepted {}
impl rdiesel::Field<Paper, bool> for schema::papers::accepted {}

#[post("/updateAccepted", format="form", data="<paper>")]
pub fn update_accepted(paper: Form<Paper>) {
    use self::schema::papers::paperid;
    use self::schema::papers::accepted;
    use self::schema::papers::dsl::*;

    let connection = &mut establish_connection_pg();

    let papers_q1 = paperid.eq(paper.paperid);
    let update_ = update_where(connection, papers_q1, accepted.assign(true));

    // diesel::update(papers)
    //     .filter(paperid.eq(paper.paperid))
    //     .set(accepted.eq(true))
    //     .execute(connection)
    //     .expect("Error updating paper");
}

impl rdiesel::Expr<AssignmentReview, i32> for schema::assignment_reviews::paperid {}
impl rdiesel::Expr<User, String> for schema::users::level {}
// displays paper from chair view
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
        let paper_q = paperid.eq(paper_id);
        let paper = select_list(connection, paper_q).expect("Error retrieving papers");

        let review_q = review_paperid.eq(paper_id);
        let reviews = select_list(connection, review_q).expect("Error retrieving reviews");
        
        let author_q = coauthor_paperid.eq(paper_id);
        let authors = select_list(connection, author_q).expect("Error retrieving authors");

        let assignment_q = assignment_paperid.eq(paper_id);
        let review_assignments = rdiesel::select_list(connection, assignment_q).expect("Error retrieving assignments");
        
        let chair_q  = level.eq("pc".to_string());
        let paper_chairs = select_list(connection, chair_q).expect("Error retrieving chairs");
        // let paper = self::schema::papers::dsl::papers  
        //     .filter(paperid.eq(paper_id))
        //     .load::<Paper>(connection)
        //     .expect("Error retrieving paper");

        // let reviews = self::schema::reviews::dsl::reviews
        //     .filter(review_paperid.eq(paper_id))
        //     .load::<Review>(connection)
        //     .expect("Error loading reviews");

        // let authors = self::schema::paper_coauthors::dsl::paper_coauthors
        //     .filter(coauthor_paperid.eq(paper_id))
        //     .load::<PaperCouthor>(connection)
        //     .expect("Error loading authors");

        // let review_assignments = self::schema::assignment_reviews::dsl::assignment_reviews
        //     .filter(assignment_paperid.eq(paper_id))
        //     .load::<AssignmentReview>(connection)
        //     .expect("Error loading Assignments reviews");

        // let paper_chairs = self::schema::users::dsl::users
        //     .filter(level.eq("pc"))
        //     .load::<User>(connection)
        //     .expect("Error loading paper chairs");

        Template::render("paper-chair", context! {paper: paper, reviews: reviews, authors: authors, review_assignments: review_assignments, paper_chairs: paper_chairs})
    } else {
        Template::render("paper-chair", context! {})
    }
}

impl rdiesel::Expr<User, i32> for schema::users::userid {}
// displays user profile
#[get("/profile/<user_id>")] 
pub fn get_user_profile(user_id: i32) -> Template {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user_q = userid.eq(user_id);
    let user = select_list(connection, user_q).expect("Error retriving user");

    // let user = self::schema::users::dsl::users
    //     .filter(userid.eq(user_id))
    //     .load::<User>(connection)
    //     .expect("Error loading user");

    Template::render("profile", context! {user})
}

// displays reviews for a paper
#[get("/review/<paper_id>")]
pub fn get_review(paper_id: i32) -> Template {
    use self::schema::reviews::paperid;

    let connection = &mut establish_connection_pg();

    let review_q = paperid.eq(paper_id);
    let reviews = select_list(connection, review_q).expect("Error retrieving reviews");

    // let reviews = self::schema::reviews::dsl::reviews
    //     .filter(paperid.eq(paper_id))
    //     .load::<Review>(connection)
    //     .expect("Error loading reviews");

    Template::render("review-show", context! {reviews})
}

impl rdiesel::Expr<Review, i32> for schema::reviews::userid {}
// displays edit form for a paper 
#[get("/review/<paper_id>/edit")]
pub fn show_edit_review(paper_id: i32, user_session: UserSession) -> Template {
    use self::schema::reviews::paperid;
    use self::schema::reviews::userid;

    let connection = &mut establish_connection_pg();

    let review_q1 = paperid.eq(paper_id);
    let review_q2 = userid.eq(user_session.user_token);
    let review_q3 = review_q1.and(review_q2);

    let review = select_list(connection, review_q3).expect("Errpr retrieving reviews");

    // let review = self::schema::reviews::dsl::reviews
    //     .filter(paperid.eq(paper_id).and(userid.eq(user_session.user_token)))
    //     .load::<Review>(connection)
    //     .expect("Error loading review");

    Template::render("paper-edit", context! {review})
}

impl rdiesel::Expr<Review, i32> for schema::reviews::reviewid {}
impl rdiesel::Expr<Review, String> for schema::reviews::content {}
impl rdiesel::Expr<Review, i32> for schema::reviews::score {}
impl rdiesel::Field<Review, String> for schema::reviews::content {}
impl rdiesel::Field<Review, i32> for schema::reviews::score {}
// posts edit to review
#[post("/review/<paper_id>/edit", format="form", data="<review>")]
pub fn edit_review(paper_id: i32, user_session: UserSession, review: Form<Review> ) -> Template {
    use self::schema::reviews::reviewid;
    use self::schema::reviews::dsl::*;
    use self::schema::reviews::userid;
    use self::schema::reviews::paperid;
    use self::schema::reviews::content;
    use self::schema::reviews::score;

    let connection = &mut establish_connection_pg();

    let review_q1 = paperid.eq(paper_id);
    let review_q2 = userid.eq(user_session.user_token);
    let review_q3 = review_q1.and(review_q2);

    let this_review = select_list(connection, review_q3).expect("Errpr retrieving reviews");

    // let this_review = self::schema::reviews::dsl::reviews
    //     .filter((paperid.eq(paper_id)).and(userid.eq(user_session.user_token)))
    //     .load::<Review>(connection)
    //     .expect("Error retrieving review");

    if !this_review.is_empty() {
        let update_q = reviewid.eq(this_review[0].reviewid);

        let _update = update_where(connection, update_q, 
            (content.assign(review.content.to_string()), score.assign(review.score)));

        // diesel::update(reviews)
        //     .filter(reviewid.eq(this_review[0].reviewid))
        //     .set((content.eq(review.content.to_string()), (score.eq(review.score))))
        //     .execute(connection)
        //     .expect("Error updating review");
    } 
    
    Template::render("paper-edit", context!{this_review})
}

// index of all papers
#[get("/index/<option>")]
pub fn get_paper_index(option: &str, user_session: UserSession) -> Template {
    use self::schema::papers::accepted;
    use self::schema::papers::author;
    use self::schema::papers::paperid;

    let connection = &mut establish_connection_pg();
    let current_user = user_session.user_token;

    let mut papers: Vec<Paper> = Vec::new(); 

    if option == "all" {
        let papers_q = paperid.gt(0);
        papers = select_list(connection, papers_q).expect("Error retrieving");

        // papers = self::schema::papers::dsl::papers
        //     .load::<Paper>(connection)
        //     .expect("Error loading papers");

    } else if option == "accepted" {
        let papers_q = accepted.eq(true);
        papers = select_list(connection, papers_q).expect("Error retriving papers");

        // papers = self::schema::papers::dsl::papers
        //     .filter(accepted.eq(true))
        //     .load::<Paper>(connection)
        //     .expect("Error loading papers");

    } else if option == "current" {
        let papers_q = author.eq(current_user);
        papers = select_list(connection, papers_q).expect("Error retriving papers");

        // papers = self::schema::papers::dsl::papers
        //     .filter(author.eq(current_user))
        //     .load::<Paper>(connection)
        //     .expect("Error loading papers");
    }

    Template::render("paper-index", context! {papers})
}

// adds paper coauthor
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

    let user_q = userid.eq(user_id);
    let user = select_list(connection, user_q).expect("Error retrieving user");

    // let user = self::schema::users::dsl::users
    //     .filter(userid.eq(user_id))
    //     .load::<User>(connection)
    //     .expect("Error loading user");
    return user
}

pub fn is_existing_user(user_id: i32) -> bool {
    use self::schema::users::userid;

    let connection = &mut establish_connection_pg();

    let user_q = userid.eq(user_id);
    let user = select_list(connection, user_q).expect("Error retrieving user");

    // let user = self::schema::users::dsl::users
    //     .filter(userid.eq(user_id))
    //     .load::<User>(connection)
    //     .expect("Error loading user");

    if user.is_empty() {
        return false;
    } else {
        return true;
    }
}