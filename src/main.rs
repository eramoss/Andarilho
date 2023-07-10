#[macro_use]
extern crate rocket;
pub mod routes;
pub mod web_walkers;
use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![amazon_search_all::get_all])
}
