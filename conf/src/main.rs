extern crate rocket;

pub mod models;
pub mod services;
pub mod schema;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![

        ])  
}