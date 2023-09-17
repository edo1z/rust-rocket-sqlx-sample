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
use sqlx::Acquire;

#[get("/")]
async fn index(mut db_con: Connection<Db>) -> Result<String, String> {
    let con = db_con.acquire().await.unwrap();
    let products = repo::find_all(con).await;
    Ok(format!("{:?}", products))
}

#[post("/new")]
async fn add(mut db_con: Connection<Db>) -> Result<String, String> {
    let con = db_con.acquire().await.unwrap();
    let mut txn = con.begin().await.unwrap();

    let user = repo::create_user(&mut txn).await;
    let product = repo::create(&mut txn).await;

    // txn.rollback().await.unwrap();
    txn.commit().await.unwrap();

    Ok(format!("product:{:?}, user:{:?}", product, user))
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, add])
}
