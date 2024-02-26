extern crate rocket;

pub mod models;
pub mod services;
pub mod schema;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            services::login,
            services::register,
            services::create_course,
            services::create_course_instructor,
            services::create_enrollment,
            services::create_assignment,
            services::create_submission
        ])  
}
