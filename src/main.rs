#[macro_use]
extern crate rocket;

pub mod app;
pub mod config;
pub mod db;

mod error {
    pub mod app_error;
    pub mod logging;
}

mod controllers {
    pub mod product_controller;
    pub mod user_controller;
}
mod use_cases {
    pub mod product_use_case;
    pub mod use_cases;
    pub mod user_use_case;
}
mod repositories {
    pub mod error;
    pub mod product_repo;
    pub mod repositories;
    pub mod user_repo;
}

mod models {
    pub mod product;
    pub mod user;
}

mod dto {
    pub mod product_dto;
    pub mod user_dto;
}

mod test {
    pub mod app;
    pub mod db;
    pub mod fixture {
        pub mod product;
        pub mod user;
    }
}

use crate::app::create_app;
use crate::config::Config;
use crate::controllers::{product_controller, user_controller};
use crate::db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[get("/")]
async fn hoge() -> &'static str {
    "hoge"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .manage(create_app())
        .mount("/", routes![hoge])
        .mount("/users", user_controller::routes())
        .mount("/products", product_controller::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_index() {
        let rocket = rocket::build()
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::hoge]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "hoge");
    }
}
