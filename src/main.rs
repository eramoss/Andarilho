#[macro_use]
extern crate rocket;
pub mod routes;
pub mod web_walkers;
use rocket::Config;
use routes::*;
use std::env;
use std::net::Ipv4Addr;
use web_walkers::start_global_pool;

#[launch]
async fn rocket() -> _ {
    tokio::time::sleep(tokio::time::Duration::new(5, 0)).await;
    start_global_pool().await.unwrap();
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
