extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::{Connection, QueryDsl, RunQueryDsl};
//use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::{json::Value, json, json::Json, Deserialize, Serialize,json::from_value,json::to_string};
use rocket::{execute, get, post };
use crate::models::{self, UserSession, User, UserDto, Course, CourseDto, CourseInstructor, CourseInstructorDto, Enrollment, 
    EnrollmentDto, Assignment, AssignmentDto, Submission, SubmissionDto, UserLogin, EnrolledCourses};
//use crate::models::{EnrollmentRequestDto, EnrollUserDto};
use crate::schema::{self, users, assignments, submissions, courses, enrollments, course_instructors};
use std::env;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::FromForm;
use rocket_dyn_templates::{context, Template};

use rdiesel::{select_list, update_where, Expr, Field};


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//User home page
#[get("/home")]
pub fn home() -> Template {
    Template::render("home",  context!{})
}

impl rdiesel::Expr<User, String> for schema::users::username {}
impl rdiesel::Expr<User, String> for schema::users::email {}


//add return
#[post("/login", format="form", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Template {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    /* 
    let is_user = self::schema::users::dsl::users
        .filter(username.eq(user.username.to_string()).and(email.eq(user.email.to_string())))
        .load::<User>(connection)
        .expect("Error loading users");
    */

    let user_q1 = username.eq(user.username.to_string());
    let user_q2 = email.eq(user.email.to_string());
    let user_q3 = user_q1.and(user_q2);

    let cur_user_wrap = rdiesel::select_list(connection, user_q3);
    let cur_user = cur_user_wrap.expect("ERROR RETRIEVING USER");


    if cur_user.is_empty() {
        Template::render("users", context! {})
    } else {
        let session_id = cur_user[0].clone().username.to_string();
        jar.add(("username", session_id));

        let user_session = UserSession {
            user_token: cur_user[0].clone().username.to_string()
        };

        //view courses will generate template
        view_courses(user_session)
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
    
    //use username as session id
    let session_id = user.username.to_string();
    jar.add(("username", session_id.clone())); //add username to cookies
}

//create course
#[post("/course", format="json", data = "<course>")]
pub fn create_course(course: Json<CourseDto>, user_session: UserSession) {
    use self::schema::courses::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    //add verification that user has instructor role
    let current_user = get_user(user_token.to_string());

    if (current_user.role == "instructor") {
        let new_course = CourseDto {
            name: course.name.to_string()
        };

        diesel::insert_into(courses)
            .values(new_course)
            .execute(connection)
            .expect("Error creating new course");
    }

}

//create course_instructor
#[post("/course/instructor", format="json", data = "<course_instructor>")]
pub fn create_course_instructor(course_instructor: Json<CourseInstructorDto>, user_session: UserSession) {
    use self::schema::course_instructors::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    //add verification that current user has instructor role
    let current_user = get_user(user_token.to_string());

    if (current_user.role == "instructor") {
        let new_course_instructor = CourseInstructorDto {
            course_id: course_instructor.course_id,
            instructor_id: course_instructor.instructor_id
        };

        diesel::insert_into(course_instructors)
            .values(new_course_instructor)
            .execute(connection)
            .expect("Error forming new course instructor pair");
    }
}

//create enrollment
#[post("/create_enrollment", format="json", data = "<enrollment>")]
pub fn create_enrollment(enrollment: Json<EnrollmentDto>, user_session: UserSession) {
    use self::schema::enrollments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    //add verification that user_session.role is instructor role
    let current_user = get_user(user_token.to_string());

    if (current_user.role == "instructor") {
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
}

//create assignment
#[post("/create_assignment", format="json", data = "<assignment>")]
pub fn create_assignment(assignment: Json<AssignmentDto>, user_session: UserSession) {
    use self::schema::assignments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    //add verification that user is instructor role
    let current_user = get_user(user_token.to_string());

    if (current_user.role == "instructor") {
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
    else {
        //return error 
    }
}

//create submission
#[post("/create_submission", format="json", data = "<submission>")]
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

/* 
//OLD, KEEPING JUST FOR REFERENCE
//view courses for user (test using json)
#[get("/view_courses_test")]
pub fn view_courses_test(user_session: UserSession) -> Json<Vec<(CourseInstructor, Course)>> {
    use self::schema::enrollments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    let current_user = get_user(user_token.to_string());
    
    //let enrolled_courses = get_enrollments(current_user.user_id);    

    let instructor_courses = get_instructor_courses(current_user.user_id);

    //return Json(enrolled_courses)
    return Json(instructor_courses)
}
*/

//final version of above method
//Returns template and different course info based on if user is a student/instructor
#[get("/view_courses")]
pub fn view_courses(user_session: UserSession) -> Template {
    use self::schema::assignments::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    let current_user = get_user(user_token.to_string());
     
     
    //two cases depending on whether user is student or teacher
    if (current_user.role == "student") {
        //get enrollments + joined course table 
        let enrolled_courses = get_enrollments(current_user.user_id);   

        //let (enrollment_data, course_data): (Vec<_>, Vec<_>) = enrolled_courses.into_iter().unzip(); 
        let (enrollment_data, course_data): (Vec<_>, Vec<_>) = enrolled_courses; 

        //add iteration to combine data into single EnrolledCourse object --> no nevermind

        //return template within if statement
        Template::render("courses", context!{courses: &course_data, data: &enrollment_data})

    }
    else if (current_user.role == "instructor") {
        //get course_instructor objects where user_id = instructor_id
        let instructor_courses = get_instructor_courses(current_user.user_id);

        //let (instructor_course_data, course_data): (Vec<_>, Vec<_>) = instructor_courses.into_iter().unzip(); 
        let (instructor_course_data, course_data): (Vec<_>, Vec<_>) = instructor_courses;
        //instructor template should display teacher's courses + have a view_assignments and view_students button
        Template::render("courses", context!{courses: &course_data, data: &instructor_course_data})
    }
    else {
        Template::render("courses", {})
    }

}


//view assignments from specific selected course - take course id from html button press
#[get("/view_assignments/<input_course_id>")]
pub fn view_assignments(input_course_id: i32, user_session: UserSession) -> Template {
    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    let current_user = get_user(user_token.to_string());
     
     //maybe provide extra info depending on whether user is student/teacher
     //like course info, grades, etc
    
    //generic so no if statement needed
    let assignments = get_assignments(input_course_id);   

        //add iteration to combine data into single EnrolledCourse object

    //return template within if statement
    Template::render("assignments", context!{assignments: &assignments, user: &current_user})


}

//view_assignments from all courses for user (to-do list maybe?)

impl rdiesel::Expr<Submission, i32> for schema::submissions::author_id {}
impl rdiesel::Expr<Submission, i32> for schema::submissions::assignment_id {}

//view submissions for a particular assignment
#[get("/view_submissions/<input_assignment_id>")]
pub fn view_submissions(input_assignment_id: i32, user_session: UserSession) -> Template {
    use self::schema::submissions::dsl::*;
    use crate::schema::submissions::assignment_id;
    use crate::schema::submissions::author_id;

    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user_token = user_session.user_token;

    let current_user = get_user(user_token.to_string());

    //if student, shows their submission | if instructor, show student submissions
    //just a vector of submissions so this can be generic -- {{each submission}} in html
    if (current_user.role == "student") {
        let submission_q1 = assignment_id.eq(input_assignment_id);
        let submission_q2 = author_id.eq(current_user.user_id);
        let submission_q3 = submission_q1.and(submission_q2);

        let user_submissions: Result<Vec<Submission>, diesel::result::Error> = rdiesel::select_list(connection, submission_q3);
        /* 
        let submissions = self::schema::submissions::dsl::submissions
            .filter(assignment_id.eq(input_assignment_id).and(author_id.eq(current_user.user_id)))
            .load::<Submission>(connection)
            .expect("Error loading submissions");
        */
        Template::render("submissions", context!{submissions: &user_submissions.expect("ERROR RETRIEVING SUBMISSIONS")})
    }
    else if (current_user.role == "instructor") {
        let submission_q1 = assignment_id.eq(input_assignment_id);

        let all_submissions: Result<Vec<Submission>, diesel::result::Error> = rdiesel::select_list(connection, submission_q1);

        /* 
        let submissions = self::schema::submissions::dsl::submissions
            .filter(assignment_id.eq(input_assignment_id))
            .load::<Submission>(connection)
            .expect("Error loading submissions");
        */
            Template::render("submissions", context!{submissions: &all_submissions.expect("ERROR RETRIEVING SUBMISSIONS")})
    }
    else {
        Template::render("courses", {})
    }
}


impl rdiesel::Expr<Submission, i32> for schema::submissions::submission_id {}

//view submission content (NOT DONE)
//assumes content is huge text block -- got to deal with file names/attachments later on
//basically acts as a redirect to a new html page with only the content
#[get("/view_submission_content/<input_submission_id>")]
pub fn view_submissions_content(input_submission_id: i32) -> Template {
    use self::schema::submissions::dsl::*;

    let connection: &mut PgConnection = &mut establish_connection_pg();   

    let submission_q1 = submission_id.eq(input_submission_id);

    let submission_content = rdiesel::select_list(connection, submission_q1);

    /* 
    let submissions = self::schema::submissions::dsl::submissions
        .filter(submission_id.eq(input_submission_id))
        .load::<Submission>(connection)
        .expect("Error loading submissions");
    */
    Template::render("submissions_content", context!{submissions: &submission_content.expect("ERROR RETRIEVING SUBMISSION CONTENT")})  
}


// Helper Functions


//get user based on username
pub fn get_user(user_name: String) -> User {
    use self::schema::users::username;

    let connection = &mut establish_connection_pg();

    let user_q1 = username.eq(user_name);

    let cur_user = rdiesel::select_list(connection, user_q1);

    /* 
    let user = self::schema::users::dsl::users
        .filter(username.eq(user_name))
        .load::<User>(connection)
        .expect("Error loading user");
    */
    return cur_user.unwrap()[0].clone()
}

impl rdiesel::Expr<Enrollment, i32> for schema::enrollments::student_id {}
impl rdiesel::Expr<Course, i32> for schema::courses::course_id {}


//get enrollments
pub fn get_enrollments(current_user_id: i32) -> (Vec<Enrollment>, Vec<Course>)  {
    //returned Vec<(Enrollment, Course)> before
    use crate::schema::enrollments::student_id;
    use crate::schema::courses::course_id;
    use self::schema::courses;


    let connection = &mut establish_connection_pg();

    /* 
    let enrolled_courses: Vec<(Enrollment, Course)> = enrollments::table
        .inner_join(courses::table)
        .filter(student_id.eq(current_user_id))
        .select((Enrollment::as_select(), Course::as_select()))
        .load::<(Enrollment, Course)>(connection)
        .expect("Error loading enrollments");

    return enrolled_courses
    */

    let enrollment_q1 = student_id.eq(current_user_id);
    let enrollment_wrap = rdiesel::select_list(connection, enrollment_q1);
    let enrollment = enrollment_wrap.expect("ERROR RETRIEVING ENROLLMENT");

    let course_q1 = course_id.eq(enrollment[0].course_id);
    let course_wrap = rdiesel::select_list(connection, course_q1);
    let course = course_wrap.expect("ERROR RETRIEVING COURSE INFO");


    return (enrollment, course)


}


impl rdiesel::Expr<CourseInstructor, i32> for schema::course_instructors::instructor_id {}

//get instructor courses
//might not need to join with Course info, just return courses
pub fn get_instructor_courses(current_user_id: i32) -> (Vec<CourseInstructor>, Vec<Course>) {
    //used to return Vec<(CourseInstructor, Course)>
    use self::schema::course_instructors::instructor_id;
    use self::schema::courses::course_id;

    let connection = &mut establish_connection_pg();

    /* 
    let instructor_courses = course_instructors::table
        .inner_join(courses::table)
        .filter(instructor_id.eq(current_user_id))
        .select((CourseInstructor::as_select(), Course::as_select()))
        .load::<(CourseInstructor, Course)>(connection)
        .expect("Error loading courses");

    return instructor_courses
    */

    let course_instructor_q1: rdiesel::Eq<i32, instructor_id, i32> = instructor_id.eq(current_user_id);
    let course_instructor_wrap = rdiesel::select_list(connection, course_instructor_q1);
    let course_instructor = course_instructor_wrap.expect("ERROR RETRIEViNG COURSE INSTRUCTOR");

    let course_q1 = course_id.eq(course_instructor[0].course_id);
    let course_wrap = rdiesel::select_list(connection, course_q1);
    let course = course_wrap.expect("ERROR RETRIEVING COURSE INFO");

    return (course_instructor, course)
}


impl rdiesel::Expr<Assignment, i32> for schema::assignments::course_id {}

//get assignments
pub fn get_assignments(input_course_id: i32) -> Vec<Assignment>{
    use self::schema::assignments;
    use crate::schema::assignments::course_id;

    let connection = &mut establish_connection_pg();

    let assignment_q1 = course_id.eq(input_course_id);
    let course_assignments_wrap = rdiesel::select_list(connection, assignment_q1);
    let course_assignments = course_assignments_wrap.expect("ERROR LOADING ASSIGNMENTS");

    /* 
    let course_assignments = self::schema::assignments::dsl::assignments
        .filter(course_id.eq(input_course_id))
        .load::<Assignment>(connection)
        .expect("Error loading assignments");
    */
    return course_assignments
}




