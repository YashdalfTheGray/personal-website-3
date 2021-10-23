use rocket_dyn_templates::Template;

#[macro_use]
extern crate rocket;

mod endpoints;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![endpoints::index])
}
