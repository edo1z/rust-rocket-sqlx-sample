#[macro_use]
extern crate rocket;

pub mod config;
pub mod db;
pub mod model;
pub mod repo;
pub mod usecase;

use config::Config;
use db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::{http::Status, State};
use rocket_db_pools::{Connection, Database};

#[get("/")]
async fn index(db_con: Connection<Db>, app: &State<AppState>) -> Result<String, (Status, String)> {
    let mut pool = db_con.into_inner();
    app.usecase
        .get_all_products_and_users(&mut pool)
        .await
        .map_err(|_| (Status::InternalServerError, "error".to_string()))
}

#[post("/new")]
async fn add(db_con: Connection<Db>, app: &State<AppState>) -> Result<String, String> {
    let mut pool = db_con.into_inner();
    app.usecase
        .create_product_and_user(&mut pool)
        .await
        .map_err(|_| "error".to_string())
}

#[get("/hoge")]
async fn hoge() -> &'static str {
    "hoge"
}

struct AppState {
    pub usecase: Box<dyn usecase::UseCase + Send + Sync>,
}

fn create_app_state() -> AppState {
    let usecase_impl = Box::new(usecase::UseCaseImpl::new());
    AppState {
        usecase: usecase_impl,
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .manage(create_app_state())
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, add, hoge])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecase::MockUseCase;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_hoge() {
        let rocket = rocket::build()
            .manage(create_app_state())
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::hoge]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/hoge").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "hoge");
    }

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_usecase = MockUseCase::new();
        mock_usecase
            .expect_get_all_products_and_users()
            .returning(|_| Ok("success".to_string()));

        let usecase_impl = Box::new(mock_usecase);
        let app_state = AppState {
            usecase: usecase_impl,
        };

        let rocket = rocket::build()
            .manage(app_state)
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "success");
    }

    #[rocket::async_test]
    async fn test_index_fail() {
        let mut mock_usecase = MockUseCase::new();
        mock_usecase
            .expect_get_all_products_and_users()
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let usecase_impl = Box::new(mock_usecase);
        let app_state = AppState {
            usecase: usecase_impl,
        };

        let rocket = rocket::build()
            .manage(app_state)
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::InternalServerError);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "error");
    }
}
