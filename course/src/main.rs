extern crate rocket;
extern crate log;

use rocket_dyn_templates::Template;
pub mod models;
pub mod services;
pub mod schema;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![services::home])
        .mount("/api", routes![
            services::login,
            services::register,
            services::create_course,
            services::create_course_instructor,
            services::create_enrollment,
            services::create_assignment,
            services::create_submission,
            services::view_courses,
            services::view_assignments
        ])  
        .attach(Template::fairing())
}
