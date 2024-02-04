extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::serde::{json::Value, json, json::Json, Deserialize, Serialize};
use rocket::{get, post };
use crate::models::{self, PasswordReset, PasswordResetDto, UserSession, User, UserDto, Group, GroupDto, Class, ClassDto, Enrollment, EnrollmentDto};
use crate::schema::{self, password_resets, users, groups, classes, enrollments};
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
#[post("/signin", format="form", data="<user>")]
pub fn sign_in(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Json<User> {
    use self::schema::users::email_address;
    use self::schema::users::password;

    let user_email_address = user.email_address.to_string();
    let user_password = user.user_password.to_string();

    let connection: &mut PgConnection = &mut establish_connection_pg();
    let current_user = self::schema::users::dsl::users
        .filter((email_address.eq(user_email_address)).and(password.eq(user_password)))
        .load::<User>(connection)
        .expect("Error");

    // need to fix this
    // need to get User object from Vec<User>
    let temp = current_user[0].clone();
    jar.add(("user-id", temp.user_id.to_string()));

    let current_user_object: User = User {
        user_id: temp.user_id.to_string(),
        email_address: temp.email_address.to_string(),
        first_name: temp.first_name.to_string(),
        last_name: temp.last_name.to_string(),
        theme: temp.theme.to_string(),
        key_binds: temp.key_binds.to_string(),
        admin: temp.admin.to_string(),
        password: temp.password.to_string()
    };

    return Json(current_user_object)
}

// post "/api/signout"        signOut
#[post("/signout")]
pub fn sign_out(jar: &CookieJar<'_>) {
    jar.remove("user_id"); 

    //return to home page
}
#[derive(Serialize, Deserialize, FromForm)]
pub struct ResetForm {
    email: String
}

// post "/api/reset"          reset
#[post("/reset", format="form", data="<password_reset>")]
pub fn create_reset(password_reset: Form<ResetForm>) {
    use self::schema::users::email_address;
    use self::schema::password_resets::dsl::*;

    let user_email = password_reset.email.to_string();

    let connection = &mut establish_connection_pg();
    let is_user = self::schema::users::dsl::users
        .filter(email_address.eq(&user_email))
        .load::<User>(connection)
        .expect("Error loading posts");

    if is_user.is_empty() {

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
    }
}

#[derive(Serialize, Deserialize, FromForm)]
pub struct PasswordResetForm {
    email: String,
    new_password: String, 
    code: String
}
// post "/api/resetpass"      resetPass
#[post("/resetpass", format="form", data="<password_reset>")]
pub fn reset_password(user_session: UserSession, password_reset: Form<PasswordResetForm>) {
    use self::schema::password_resets::code;
    use self::schema::users::dsl::*;
    use self::schema::password_resets::dsl::*;
    use crate::schema::password_resets::valid;

    let reset_code = password_reset.code.to_string();
    let new_password = password_reset.new_password.to_string();

    let connection = &mut establish_connection_pg();

    let is_code_valid = self::schema::password_resets::dsl::password_resets
        .filter(code.eq(&reset_code).and(valid.eq(true)))
        .load::<PasswordReset>(connection)
        .expect("Error retrieving");

    if is_code_valid.is_empty() {

    } else {
        let current_user_id = user_session.user_token;

        diesel::update(users)
            .filter(user_id.eq(current_user_id))
            .set(password.eq(new_password))
            .execute(connection)
            .expect("Error Updating");

        diesel::update(password_resets)
            .filter(code.eq(&reset_code))
            .set(valid.eq(false))
            .execute(connection)
            .expect("Error updating"); 
    }
}
// get  "/api/user/me"        userGetMe
// post "/api/enroll"         addRoster
// post "/api/setlanguage"    setLanguage
// get  "/api/roster/:class"  getRoster

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
pub fn add_class(jar: &CookieJar<'_>, classDto: Json<ClassDto>) -> Json<String> {
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
pub fn add_group(jar: &CookieJar<'_>, groupDto: Json<GroupDto>) -> Json<String> {
    use self::schema::groups::dsl::*;
    use crate::models::GroupDto;
    let connection = &mut establish_connection_pg();

    diesel::insert_into(groups)
        .values(groupDto.into_inner())
        .execute(connection)
        .expect("Error saving new user");


    return Json("Successfully added group".to_string())
}


//post addRoster
//post getRoster
//post setLanguage
//post genRandomText
//post sendMail






