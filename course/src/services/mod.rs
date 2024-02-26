extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::{json::Value, json, json::Json, Deserialize, Serialize,json::from_value,json::to_string};
use rocket::{execute, get, post };
use crate::models::{self, UserSession, User, UserDto, Course, CourseDto, CourseInstructor, CourseInstructorDto, Enrollment, 
    EnrollmentDto, Assignment, AssignmentDto, Submission, SubmissionDto};
//use crate::models::{EnrollmentRequestDto, EnrollUserDto};
use crate::schema::{self, users, assignments, submissions, courses, enrollments, course_instructors};
use std::env;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::FromForm;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//add return
#[post("/login", format="json", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Json<User>){
    use self::schema::users::dsl::*;

    let user_username = user.username.to_string();
    let user_email = user.email.to_string();
    let connection = &mut establish_connection_pg();

    let is_user = self::schema::users::dsl::users
        .filter((username.eq(user_username)).and(email.eq(user_email)))
        .load::<User>(connection)
        .expect("Error loading users");

    if is_user.is_empty() {
        //add
    } else {
        let session_id = is_user[0].clone().user_id.to_string();
        jar.add(("user_id", session_id));
    }
}

//need to decide between json or form data

#[post("/register", format="json", data = "<user>")]
pub fn register(jar: &CookieJar<'_>, user: Json<UserDto>) {
    use self::schema::users::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let new_user = UserDto {
        username: user.username.to_string(),
        email: user.email.to_string(),
        name: user.name.to_string(),
        role: user.role.to_string()
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error creating new user");

    //create helper function for retrieving user object in database, then use returned generated id as session_id
    //let session_id = user.user_id.to_string();
    //jar.add(("user_id", session_id.clone())); //add user_id to cookies
}

//create course
#[post("/course", format="json", data = "<course>")]
pub fn create_course(course: Json<CourseDto>, user_session: UserSession) {
    use self::schema::courses::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = &user_session.user_token;

    //add verification that user_session.role has instructor role

    let new_course = CourseDto {
        name: course.name.to_string()
    };

    diesel::insert_into(courses)
        .values(new_course)
        .execute(connection)
        .expect("Error creating new course");

}

//create course_instructor
#[post("/course/instructor", format="json", data = "<course_instructor>")]
pub fn create_course_instructor(course_instructor: Json<CourseInstructorDto>, user_session: UserSession) {
    use self::schema::course_instructors::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = &user_session.user_token;

    //add verification that user_session.role has instructor role

    let new_course_instructor = CourseInstructorDto {
        course_id: course_instructor.course_id,
        instructor_id: course_instructor.instructor_id
    };

    diesel::insert_into(course_instructors)
        .values(new_course_instructor)
        .execute(connection)
        .expect("Error forming new course instructor pair");
}

//create enrollment
#[post("/enrollment", format="json", data = "<enrollment>")]
pub fn create_enrollment(enrollment: Json<EnrollmentDto>, user_session: UserSession) {
    use self::schema::enrollments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = &user_session.user_token;

    //add verification that user_session.role is instructor role


    let new_enrollment = EnrollmentDto {
        student_id: enrollment.student_id,
        course_id: enrollment.course_id,
        grade: enrollment.grade.to_string()
    };

    diesel::insert_into(enrollments)
        .values(new_enrollment)
        .execute(connection)
        .expect("Error creating new enrollment");

}

//create assignment
#[post("/assignment", format="json", data = "<assignment>")]
pub fn create_assignment(assignment: Json<AssignmentDto>, user_session: UserSession) {
    use self::schema::assignments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = &user_session.user_token;

    //add verification that user_session.role is instructor role


    let new_assignment = AssignmentDto {
        name: assignment.name.to_string(),
        course_id: assignment.course_id,
        description: assignment.description.to_string()
    };

    diesel::insert_into(assignments)
        .values(new_assignment)
        .execute(connection)
        .expect("Error creating new enrollment");

}

//create submission
#[post("/submission", format="json", data = "<submission>")]
pub fn create_submission(submission: Json<SubmissionDto>, user_session: UserSession) {
    use self::schema::submissions::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();


    let new_submission = SubmissionDto {
        assignment_id: submission.assignment_id,
        author_id: submission.author_id,
        content: submission.content.to_string(),
        grade: submission.grade.to_string()
    };

    diesel::insert_into(submissions)
        .values(new_submission)
        .execute(connection)
        .expect("Error creating new enrollment");

}


//view courses + name,gpa for user

//view assignments from all courses (+ specific course) for user

//view submissions for user (and for instructors), both all and for specific courses    

//
