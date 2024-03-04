extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::{json::Value, json, json::Json, Deserialize, Serialize,json::from_value,json::to_string};
use rocket::{execute, get, post };
use crate::models::{self, PasswordReset, PasswordResetDto, UserSession, User, UserDto, Group, GroupDto, Class, ClassDto, Enrollment, EnrollmentDto};
use crate::models::{EnrollmentRequestDto, EnrollUserDto};
use crate::schema::{self, password_resets, users, groups, classes, enrollments};
use std::alloc::System;
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

#[derive(Serialize, Deserialize, FromForm)]
pub struct UserLogin {
    pub email_address: String,
    pub user_password: String
}

// post "/api/signin"         signIn
#[post("/signin", format="json", data="<user>")]
pub fn sign_in(jar: &CookieJar<'_>, user: Json<UserLogin>) -> Json<User> {
    use self::schema::users::email_address;
    use self::schema::users::password;

    let user_email_address = user.email_address.to_string();
    let user_password = user.user_password.to_string();

    let connection: &mut PgConnection = &mut establish_connection_pg();
    let current_user = self::schema::users::dsl::users
        .filter((email_address.eq(user_email_address)).and(password.eq(user_password)))
        .load::<User>(connection)
        .expect("Error");


    jar.add(("user_id", current_user[0].clone().user_id.to_string()));

    return Json(current_user[0].clone())
}

// post "/api/signout"        signOut
#[post("/signout")]
pub fn sign_out(jar: &CookieJar<'_>) -> Json<String> {
    jar.remove("user_id"); 

    return Json("User logged out".to_string())
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct ResetForm {
    email: String
}

// post "/api/reset"          reset
#[post("/reset", format="json", data="<password_reset>")]
pub fn create_reset(password_reset: Json<ResetForm>) -> Json<String> {
    use self::schema::users::email_address;
    use self::schema::password_resets::dsl::*;

    let user_email = password_reset.email.to_string();

    let connection = &mut establish_connection_pg();
    let is_user = self::schema::users::dsl::users
        .filter(email_address.eq(&user_email))
        .load::<User>(connection)
        .expect("Error loading posts");

    if is_user.is_empty() {
        return Json("User with given email address is not found".to_string())
    } else {
        let new_reset = PasswordResetDto {
            email: user_email,
            code: String::from("ABCDE"),
            valid: true,
            unique_request: String::from("HI"),
        };

        diesel::insert_into(password_resets)
            .values(new_reset)
            .execute(connection)
            .expect("Error creating reset");

        return Json("Check given email".to_string())
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct PasswordResetForm {
    email: String,
    new_password: String, 
    code: String
}

// can update this to use cookies instead 
// post "/api/resetpass"      resetPass
#[post("/resetpass", format="json", data="<password_reset>")]
pub fn reset_password(password_reset: Json<PasswordResetForm>) -> Json<String> {
    use self::schema::password_resets::code;
    use self::schema::users::dsl::*;
    use self::schema::password_resets::dsl::*;
    use crate::schema::password_resets::valid;
    use self::schema::users::email_address;

    let reset_code = password_reset.code.to_string();
    let new_password = password_reset.new_password.to_string();
    let user_email = password_reset.email.to_string();

    let connection = &mut establish_connection_pg();

    let is_code_valid = self::schema::password_resets::dsl::password_resets
        .filter(code.eq(&reset_code))
        .load::<PasswordReset>(connection)
        .expect("Error retrieving");

    if is_code_valid.is_empty() {
        return Json("code is invalid".to_string())
    } else {
        // let current_user_id = user_session.user_token;

        diesel::update(users)
            .filter(email_address.eq(user_email))
            .set(password.eq(new_password))
            .execute(connection)
            .expect("Error Updating");

        diesel::update(password_resets)
            .filter(code.eq(&reset_code))
            .set(valid.eq(false))
            .execute(connection)
            .expect("Error updating"); 

        return Json("Succesfully reset password".to_string())
    }
}

//can change to take class_id as json input if needed
//post addEnrollment - enrollm a student/user into a class + group. user follows format of EnrollUserDto
#[post("/enroll", format="json", data= "<enroll>")]
pub fn add_enrollment(jar: &CookieJar<'_>, enroll: Json<EnrollmentRequestDto>) -> Json<String> {
    use self::schema::users::dsl::*;
    use self::schema::enrollments::dsl::*;

    let connection = &mut establish_connection_pg();

    let enroll = enroll.into_inner();

    let a_class_id = enroll.class_id;

    let mut user = &enroll.users[0];

    if (!is_existing_user(user.user_id.to_string())) {
        let new_user = UserDto {
            user_id: user.user_id.to_string(),
            email_address: user.email_address.to_string(),
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            theme: "default".to_string(),
            key_binds: "k".to_string(),
            admin: "false".to_string(),
            password: user.password.to_string()
        };

        add_user(jar, Json(new_user));
    }
    //for testing
    else {
        println!("User {:?}", user.user_id.to_string());
    }

    let new_enroll = EnrollmentDto {
        user_id: user.user_id.to_string(),
        class_id: a_class_id,
        group_id: user.group_id
    };

    diesel::insert_into(enrollments)
        .values(&new_enroll)
        .execute(connection)
        .expect("Error saving new user");

    return Json("Successfully enrolled student".to_string())
    
}


//post addRoster, vector of enrollments, change data... 
#[post("/add_roster", format="json", data= "<rosterRequest>")]
pub fn add_roster(jar: &CookieJar<'_>, user_session: UserSession, rosterRequest: Json<EnrollmentRequestDto>) -> Json<String> {
    //todo: check if user_session is authorized instructor
    use self::schema::users::dsl::*;
    use self::schema::enrollments::dsl::*;

    let connection = &mut establish_connection_pg();

    let roster = rosterRequest.into_inner();

    let a_class_id = roster.class_id;

    let usersList: Vec<EnrollUserDto> = roster.users;
    for user in usersList{
        //test
        println!("{:?}", a_class_id);
        println!("{:?}", user.last_name.to_string());
        //todo: check if user is in database. call following line only if not in database.
        if (!is_existing_user(user.user_id.to_string())) {
            let new_user = UserDto {
                user_id: user.user_id.to_string(),
                email_address: user.email_address.to_string(),
                first_name: user.first_name.to_string(),
                last_name: user.last_name.to_string(),
                theme: "default".to_string(),
                key_binds: "k".to_string(),
                admin: "false".to_string(),
                password: user.password.to_string()
            };

            add_user(jar, Json(new_user));
        }
        //for testing
        else {
            println!("User {:?}", user.user_id.to_string());
        }

        let new_enroll = EnrollmentDto {
            user_id: user.user_id.to_string(),
            class_id: a_class_id,
            group_id: user.group_id
        };

        diesel::insert_into(enrollments)
            .values(&new_enroll)
            .execute(connection)
            .expect("Error saving new user");

    }

    return Json("Successfully added roster".to_string())
}


//get getRoster
#[get("/roster/<a_class_id>")]
pub fn get_roster(a_class_id: i32, user_session: UserSession) -> Json<Vec<User>> {
    use self::schema::enrollments::dsl::*;
    use self::schema::enrollments::class_id;
    use self::schema::users::user_id;
       
    let connection = &mut establish_connection_pg();

    let all_enrolls = self::schema::enrollments::dsl::enrollments
        .filter(class_id.eq(a_class_id))
        .load::<Enrollment>(connection)
        .expect("Error loading roster");

    //iterate through enrollment vector and find their corresponding user objects, then return
    let mut users: Vec<User> = Vec::new();

    for enroll in all_enrolls {
        let user = self::schema::users::dsl::users
            .filter(user_id.eq(enroll.user_id))
            .load::<User>(connection)
            .expect("Error loading user");

        users.push(user[0].clone());
    }

    return Json(users)
}



//post addUser
#[post("/add_user", format="json", data = "<user>")]
pub fn add_user(jar: &CookieJar<'_>, user: Json<UserDto>) -> Json<UserDto> {
    //allow an instructor to add a student user and add it to the database. 
    use self::schema::users::dsl::*;
    use crate::models::UserDto;
    let connection = &mut establish_connection_pg();

    let new_user = UserDto {
        user_id: user.user_id.to_string(),
        email_address: user.email_address.to_string(),
        first_name: user.first_name.to_string(),
        last_name: user.last_name.to_string(),
        theme: user.theme.to_string(),
        key_binds: user.key_binds.to_string(),
        admin: user.admin.to_string(),
        password: user.password.to_string()
    };

    let result = diesel::insert_into(users)
        //.values(user.into())
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    let session_user_id = user.user_id.to_string();
    println!("Your user_id: {}", session_user_id);
    jar.add(("user_id", session_user_id.clone()));

    return Json(new_user)
}

//post addClass
#[post("/add_class", format="json", data = "<classDto>")]
pub fn add_class(classDto: Json<ClassDto>) -> Json<String> {
    use self::schema::classes::dsl::*;
    use crate::models::ClassDto;
    let connection = &mut establish_connection_pg();

    diesel::insert_into(classes)
        .values(classDto.into_inner())
        .execute(connection)
        .expect("Error saving new user");

    return Json("Successfully added class".to_string())
}

//post addGroup
#[post("/add_group", format="json", data = "<groupDto>")]
pub fn add_group(groupDto: Json<GroupDto>) -> Json<String> {
    use self::schema::groups::dsl::*;
    use crate::models::GroupDto;
    let connection = &mut establish_connection_pg();

    diesel::insert_into(groups)
        .values(groupDto.into_inner())
        .execute(connection)
        .expect("Error saving new user");


    return Json("Successfully added group".to_string())
}

// get  "/api/user/me"        userGetMe
#[get("/user")]
pub fn get_user(user_session: UserSession) -> Json<User> {
    use self::schema::users::dsl::*;
    let connection = &mut establish_connection_pg();

    let user_token = &user_session.user_token.to_string();

    let current_user = self::schema::users::dsl::users
        .filter(user_id.eq(user_token))
        .load::<User>(connection)
        .expect("Error loading user");

    return Json(current_user[0].clone())
}

pub fn is_existing_user(a_user_id: String) -> bool {
    use self::schema::users::dsl::*;
    let connection = &mut establish_connection_pg();


    let db_user = self::schema::users::dsl::users
        .filter(user_id.eq(a_user_id))
        .load::<User>(connection)
        .expect("Error finding user");

    if (db_user.len() > 0) {
        return true;
    }
    else {
        return false;
    }
}

//post setLanguage
#[post("/setLanguage", format="json", data="<class>")]
pub fn change_language(class: Json<Class>) -> Json<String> {
    use self::schema::classes::dsl::*;

    let connection = &mut establish_connection_pg();

    diesel::update(classes)
        .filter(class_id.eq(class.clone().into_inner().class_id))
        .set(editor_language.eq(class.clone().into_inner().editor_language))
        .execute(connection)
        .expect("Error updating language");

    return Json("Language updated successfully".to_string())

}
//post genRandomText


//post sendMail






