#[macro_use]
extern crate rocket;
pub mod routes;
pub mod web_walkers;
use std::net::Ipv4Addr;

use rocket::Config;
use routes::*;

#[launch]
fn rocket() -> _ {
    let mut config = Config::default();
    config.address = Ipv4Addr::new(0, 0, 0, 0).try_into().unwrap();
    rocket::build()
        .mount("/", routes![amazon_routes::get_all])
        .configure(config)
}
