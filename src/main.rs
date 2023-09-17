#[macro_use]
extern crate rocket;

pub mod config;
pub mod db;
pub mod model;
pub mod repo;

use config::Config;
use db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;

#[get("/")]
async fn index(db_con: Connection<Db>) -> Result<String, String> {
    let products = repo::find_all(db_con).await;
    Ok(format!("{:?}", products))
}

#[post("/new")]
async fn add(db_con: Connection<Db>) -> Result<String, String> {
    let product = repo::create(db_con).await;
    Ok(format!("{:?}", product))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, add])
}
