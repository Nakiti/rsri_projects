extern crate rocket;

pub mod models;
pub mod services;
pub mod schema;
use rocket_dyn_templates::Template;
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            services::login,
            services::get_login_page,
            services::register,
            services::get_register_page,
            services::get_user,
            services::get_paper,
            services::create_paper,
            services::show_paper,
            services::get_paper_edit,
            services::edit_paper,
            services::create_assignment_review,
            services::show_paper_chair,
            services::get_user_profile,
            services::get_review,
            services::get_paper_index,
            services::create_paper_coauthor,
            services::logout,
            services::get_logout_page
        ])  
        .attach(Template::fairing())
}
