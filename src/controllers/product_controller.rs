use crate::app::AppState;
use crate::db::ConnectionDb;
use crate::dto::product_dto::ProductName;
use crate::error::app_error::AppError;
use crate::models::product_model::Product;
use rocket::serde::json::Json;
use tracing::instrument;

#[get("/")]
#[instrument(name = "product_controller/index", skip_all)]
async fn index(app: &AppState, mut db: ConnectionDb) -> Result<Json<Vec<Product>>, AppError> {
    let products = app.use_cases.product.find_all(&app.repos, &mut db).await?;
    Ok(Json(products))
}

#[post("/add", data = "<name>")]
#[instrument(name = "product_controller/add", skip_all)]
async fn add(
    app: &AppState,
    mut db: ConnectionDb,
    name: Json<ProductName>,
) -> Result<Json<Product>, AppError> {
    let name = name.into_inner().name;
    let product = app
        .use_cases
        .product
        .create(&app.repos, &mut db, &name)
        .await?;
    Ok(Json(product))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, add]
}

#[cfg(test)]
mod tests {
    use crate::app_err;
    use crate::config::Config;
    use crate::db::Db;
    use crate::test::app::create_app_for_test;
    use crate::test::fixture::product::products_fixture;
    use crate::use_cases::product_use_case::MockProductUseCase;
    use rocket::fairing::AdHoc;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket_db_pools::Database;
    use std::sync::Arc;

    #[rocket::async_test]
    async fn test_index_success() {
        let mut mock_product_use_case = MockProductUseCase::new();
        mock_product_use_case
            .expect_find_all()
            .returning(|_, _| Ok(products_fixture(5)));

        let mut app_state = create_app_for_test();
        app_state.use_cases.product = Box::new(mock_product_use_case);

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
        let mut mock_product_use_case = MockProductUseCase::new();
        mock_product_use_case
            .expect_find_all()
            .returning(|_, _| app_err!(500, "error"));

        let mut app_state = create_app_for_test();
        app_state.use_cases.product = Box::new(mock_product_use_case);

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
