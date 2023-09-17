use crate::db::Db;
use crate::model::Product;
use rocket_db_pools::Connection;

pub async fn find_all(mut db_con: Connection<Db>) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&mut **db_con)
        .await?;
    Ok(products)
}
