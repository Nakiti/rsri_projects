extern crate rocket;

pub mod models;
pub mod services;
pub mod schema;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            services::sign_in,
            services::sign_out,
            services::add_user,
            services::add_group,
            services::add_class
        ])  
}
