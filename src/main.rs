#[macro_use]
extern crate rocket;

mod endpoints;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![endpoints::index])
}
