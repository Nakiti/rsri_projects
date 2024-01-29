extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::Debug;
use rocket::{get, post};
use std::env;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// -- Specified in voltron/server.hs
// post "/api/signin"         signIn
// post "/api/signout"        signOut
// post "/api/reset"          reset
// post "/api/resetpass"      resetPass
// get  "/api/user/me"        userGetMe
// post "/api/enroll"         addRoster
// post "/api/setlanguage"    setLanguage
// get  "/api/roster/:class"  getRoster




