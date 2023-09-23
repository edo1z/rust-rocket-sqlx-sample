#[cfg(test)]
use crate::db::DbCon;
#[cfg(test)]
use dotenv::dotenv;
#[cfg(test)]
use sqlx::{postgres::PgPoolOptions, Error};
#[cfg(test)]
use std::env;

#[cfg(test)]
pub async fn create_db_con_for_test() -> Result<DbCon, Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
    db_pool.acquire().await
}
