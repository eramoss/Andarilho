#[macro_use]
extern crate rocket;

use andarilho::routes::amazon_routes;
use andarilho::wd_pool::init_global_pool;
use rocket::Config;
use std::env;
use std::net::Ipv4Addr;

#[launch]
async fn rocket() -> _ {
    init_global_pool().await;

    let mut config = Config::default();
    config.address = Ipv4Addr::new(0, 0, 0, 0).try_into().unwrap();
    config.port = match env::var("ROCKET_PORT") {
        Ok(port) => port
            .parse()
            .expect("Couldn't parse port to u16, make sure to use only numbers, Example: 8000"),
        Err(_) => {
            println!(
                "Couldn't get port on environment variable, make sure you have a ROCKET_PORT var."
            );
            8000
        }
    };
    rocket::build()
        .mount("/", routes![amazon_routes::get_all])
        .configure(config)
}
