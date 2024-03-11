extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
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

//add return
#[post("/login", format="form", data="<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Template {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let is_user = self::schema::users::dsl::users
        .filter(username.eq(user.username.to_string()).and(email.eq(user.email.to_string())))
        .load::<User>(connection)
        .expect("Error loading users");

    if (is_user.is_empty()) {
        Template::render("users", context! {})
    } else {
        let session_id = is_user[0].clone().username.to_string();
        jar.add(("username", session_id));

        let user_session = UserSession {
            user_token: is_user[0].clone().username.to_string()
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

//to be created- final version of above method
//it will return template probably, return different info based on student/instructor
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

        let (enrollment_data, course_data): (Vec<_>, Vec<_>) = enrolled_courses.into_iter().unzip(); 

        //add iteration to combine data into single EnrolledCourse object

        //return template within if statement
        Template::render("courses", context!{courses: &course_data, data: &enrollment_data})

    }
    else if (current_user.role == "instructor") {
        //get course_instructor objects where user_id = instructor_id
        let instructor_courses = get_instructor_courses(current_user.user_id);

        let (instructor_course_data, course_data): (Vec<_>, Vec<_>) = instructor_courses.into_iter().unzip(); 

        //instructor template should display teacher's courses + have a view_assignments and view_students button
        Template::render("courses", context!{courses: &course_data, data: &instructor_course_data})
    }
    else {
        Template::render("courses", {})
    }

}


//view assignments from specific selected course - take course id from html button press

//to be created- final version of above method
//it will return template probably, return different info based on student/instructor
#[get("/view_assignments/<input_course_id>")]
pub fn view_assignments(input_course_id: i32, user_session: UserSession) -> Template {
    use self::schema::enrollments::dsl::*;

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

//view submissions for user (and for instructors), both all and for specific courses    


// Helper Functions

//get user based on username
pub fn get_user(user_name: String) -> User {
    use self::schema::users::username;

    let connection = &mut establish_connection_pg();

    let user = self::schema::users::dsl::users
        .filter(username.eq(user_name))
        .load::<User>(connection)
        .expect("Error loading user");

    return user[0].clone()
}

//get enrollments
pub fn get_enrollments(current_user_id: i32) -> Vec<(Enrollment, Course)> {
    use crate::schema::enrollments::student_id;
    use self::schema::courses;


    let connection = &mut establish_connection_pg();

    let enrolled_courses: Vec<(Enrollment, Course)> = enrollments::table
        .inner_join(courses::table)
        .filter(student_id.eq(current_user_id))
        .select((Enrollment::as_select(), Course::as_select()))
        .load::<(Enrollment, Course)>(connection)
        .expect("Error loading enrollments");

    return enrolled_courses
}

//get instructor courses
//might not need to join with Course info, just return courses
pub fn get_instructor_courses(current_user_id: i32) -> Vec<(CourseInstructor, Course)> {
    use self::schema::course_instructors::instructor_id;

    let connection = &mut establish_connection_pg();

    let instructor_courses = course_instructors::table
        .inner_join(courses::table)
        .filter(instructor_id.eq(current_user_id))
        .select((CourseInstructor::as_select(), Course::as_select()))
        .load::<(CourseInstructor, Course)>(connection)
        .expect("Error loading courses");

    return instructor_courses
}

//get courses
pub fn get_assignments(input_course_id: i32) -> Vec<Assignment>{
    use self::schema::assignments;
    use crate::schema::assignments::course_id;

    let connection = &mut establish_connection_pg();

    let assignments = self::schema::assignments::dsl::assignments
        .filter(course_id.eq(course_id))
        .load::<Assignment>(connection)
        .expect("Error loading assignments");

    return assignments;
}





