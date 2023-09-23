use crate::app::AppState;
use crate::db::ConnectionDb;
use rocket::http::Status;

#[get("/")]
async fn index(app: &AppState, mut db: ConnectionDb) -> Result<String, (Status, String)> {
    let users = app
        .use_cases
        .user
        .get_all(&app.repos, &mut db)
        .await
        .map_err(|_| (Status::InternalServerError, "error".to_string()))?;
    Ok(users.len().to_string())
}

#[post("/new")]
async fn add(app: &AppState, mut db: ConnectionDb) -> Result<String, String> {
    let user = app
        .use_cases
        .user
        .create(&app.repos, &mut db)
        .await
        .map_err(|_| "error".to_string())?;
    Ok(user.name)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add]
}

#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::db::Db;
    use crate::test::app::create_app_for_test;
    use crate::test::fixture::user::users_fixture;
    use crate::use_cases::user_use_case::MockUserUseCase;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::Database;
    use std::sync::Arc;

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_get_all()
            .returning(|_, _| Ok(users_fixture(5)));

        let mut app_state = create_app_for_test();
        app_state.use_cases.user = Box::new(mock_user_use_case);

        let rocket = rocket::build()
            .manage(Arc::new(app_state))
            .attach(Db::init())
            .attach(AdHoc::config::<Config>())
            .mount("/", routes![super::index]);
        let client = Client::tracked(rocket)
            .await
            .expect("valid rocket instance");
        let response = client.get("/").dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        let body_str = response.into_string().await.expect("valid body string");
        assert_eq!(body_str, "5");
    }

    #[rocket::async_test]
    async fn test_index_fail() {
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_get_all()
            .returning(|_, _| Err(sqlx::Error::RowNotFound));

        let mut app_state = create_app_for_test();
        app_state.use_cases.user = Box::new(mock_user_use_case);

        let rocket = rocket::build()
            .manage(Arc::new(app_state))
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
