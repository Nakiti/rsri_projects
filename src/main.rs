extern crate rocket;

pub mod models;
pub mod services;
pub mod schema;

use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
}
