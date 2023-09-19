use crate::app_state::AppState;
use crate::db::Db;
use rocket::{http::Status, State};
use rocket_db_pools::Connection;

#[get("/")]
async fn index(db_con: Connection<Db>, app: &State<AppState>) -> Result<String, (Status, String)> {
    let mut pool = db_con.into_inner();
    app.use_case
        .user
        .get_all(&mut pool, &app)
        .await
        .map_err(|_| (Status::InternalServerError, "error".to_string()))
}

#[post("/new")]
async fn add(db_con: Connection<Db>, app: &State<AppState>) -> Result<String, String> {
    let mut pool = db_con.into_inner();
    app.use_case
        .user
        .create(&mut pool, &app)
        .await
        .map_err(|_| "error".to_string())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::test::app_state::create_app_state_for_test;
    use crate::use_cases::user_use_case::MockUserUseCase;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::Database;

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_get_all()
            .returning(|_, _| Ok("success".to_string()));

        let mut app_state = create_app_state_for_test();
        app_state.use_case.user = Box::new(mock_user_use_case);

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
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_get_all()
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

        let mut app_state = create_app_state_for_test();
        app_state.use_case.user = Box::new(mock_user_use_case);

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
