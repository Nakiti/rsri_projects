extern crate rocket;

use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
}
