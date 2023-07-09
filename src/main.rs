#[macro_use]
extern crate rocket;
pub mod routes;
pub mod web_walkers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index])
}
