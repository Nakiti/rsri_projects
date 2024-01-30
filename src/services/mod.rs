extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post };
use crate::models::{self, UserLogin};
use crate::schema;
use rocket_dyn_templates::{context, Template};
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
pub fn signIn(jar: &CookieJar<'_>, user: Form<UserLogin>) -> Json<User> {
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
// post "/api/reset"          reset
// post "/api/resetpass"      resetPass
// get  "/api/user/me"        userGetMe
// post "/api/enroll"         addRoster
// post "/api/setlanguage"    setLanguage
// get  "/api/roster/:class"  getRoster




