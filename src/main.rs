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

#[get("/hoge")]
async fn hoge() -> &'static str {
    "hoge"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, add, hoge])
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_hoge() {
        // Rocketインスタンスを作成
        let rocket = rocket::build().mount("/", routes![super::hoge]);

        // テスト用のClientを作成
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");

        // GET /hoge リクエストを模倣
        let response = client.get("/hoge").dispatch().await;

        // レスポンスステータスが200 OKであることを確認
        assert_eq!(response.status(), Status::Ok);

        // レスポンスボディが "hoge" であることを確認
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "hoge");
    }
}
