extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::{connection, prelude::*};
use dotenvy::dotenv;
use rocket::http::hyper::server::conn;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post };
use crate::models::{self, PasswordReset, PasswordResetDto, UserLogin};
use crate::schema::{self, password_resets};
use crate::schema::password_resets::email;
use std::env;
use rocket::form::Form;
use crate::models::{User};
use rocket::http::CookieJar;


pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// post "/api/signin"         signIn
#[post("/signin", format="form", data="<user>")]
pub fn sign_in(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Json<User> {
    use self::schema::users::email_address;
    use self::schema::users::password;

    let connection: &mut PgConnection = &mut establish_connection_pg();
    let current_user = self::schema::users::dsl::users
        .filter((email_address.eq(user.email_address.to_string())).and(password.eq(user.password.to_string())))
        .load::<User>(connection)
        .expect("Error");

    // need to fix this
    // need to get User object from Vec<User>
    let temp = current_user[0].clone();
    jar.add(("user-id", temp.user_id.to_string()));

    let current_user_object: User = User {
        user_id: temp.user_id,
        email_address: temp.email_address,
        first_name: temp.first_name,
        last_name: temp.last_name,
        theme: temp.theme,
        key_binds: temp.key_binds,
        admin: temp.admin,
        password: temp.password
    };

    return Json(current_user_object)
}


// post "/api/signout"        signOut
#[post("/signout")]
pub fn sign_out(jar: &CookieJar<'_>) {
    jar.remove("user_id"); 

    //return to home page
}

// post "/api/reset"          reset
#[post("/reset", format="form", data="<password_reset>")]
pub fn reset(jar: &CookieJar<'_>, password_reset: Form<PasswordResetDto>) {
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
// post "/api/resetpass"      resetPass
// get  "/api/user/me"        userGetMe
// post "/api/enroll"         addRoster
// post "/api/setlanguage"    setLanguage
// get  "/api/roster/:class"  getRoster




