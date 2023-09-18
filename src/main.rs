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
use rocket_db_pools::{Connection, Database};
use sqlx::Acquire;

#[get("/")]
async fn index(db_con: Connection<Db>) -> Result<String, String> {
    let mut pool = db_con.into_inner();
    let products = repo::find_all_products(&mut pool).await;
    let users = repo::find_all_users(&mut pool).await;
    Ok(format!("products: {:?} usres: {:?}", products, users))
}

#[post("/new")]
async fn add(mut db_con: Connection<Db>) -> Result<String, String> {
    let mut txn = db_con.begin().await.unwrap();

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
