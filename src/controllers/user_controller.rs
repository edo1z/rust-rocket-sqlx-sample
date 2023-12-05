use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::user_dto::UserName;
use crate::error::app_error::AppError;
use crate::models::user_model::User;
use rocket::serde::json::Json;
use tracing::instrument;

#[get("/")]
#[instrument(name = "user_controller/index", skip_all)]
async fn index(app: &AppState, mut db: ConnectionDb) -> Result<Json<Vec<User>>, AppError> {
    let users = app.use_cases.user.find_all(&app.repos, &mut db).await?;
    Ok(Json(users))
}

#[post("/add", data = "<name>")]
#[instrument(name = "user_controller/add", skip_all)]
async fn add(
    app: &AppState,
    mut db: ConnectionDb,
    name: Json<UserName>,
) -> Result<Json<User>, AppError> {
    let name = name.into_inner().name;
    let user = app
        .use_cases
        .user
        .create(&app.repos, &mut db, &name)
        .await?;
    Ok(Json(user))
}

#[put("/<id>", data = "<name>")]
#[instrument(name = "user_controller/update", skip_all, fields(id = %id))]
async fn update(
    app: &AppState,
    mut db: ConnectionDb,
    id: i32,
    name: Json<UserName>,
) -> Result<Json<User>, AppError> {
    let name = name.into_inner().name;
    let user = app
        .use_cases
        .user
        .update(&app.repos, &mut db, id, &name)
        .await?;
    Ok(Json(user))
}

#[delete("/<id>")]
#[instrument(name = "user_controller/delete", skip_all, fields(id = %id))]
async fn delete(app: &AppState, mut db: ConnectionDb, id: i32) -> Result<(), AppError> {
    app.use_cases.user.delete(&app.repos, &mut db, id).await?;
    Ok(())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add, update, delete]
}

#[cfg(test)]
mod tests {
    use crate::app_err;
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
            .expect_find_all()
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
    }

    #[rocket::async_test]
    async fn test_index_fail() {
        let mut mock_user_use_case = MockUserUseCase::new();
        mock_user_use_case
            .expect_find_all()
            .returning(|_, _| app_err!(500, "error!"));

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
        assert_eq!(body_str, "error!");
    }
}
