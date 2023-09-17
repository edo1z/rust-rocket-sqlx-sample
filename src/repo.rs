use crate::db::Db;
use crate::model::Product;
use rocket_db_pools::Connection;

pub async fn find_all(mut db_con: Connection<Db>) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&mut **db_con)
        .await?;
    Ok(products)
}

pub async fn create(mut db_con: Connection<Db>) -> Result<Product, sqlx::Error> {
    let product = sqlx::query_as::<_, Product>("INSERT INTO products (name) VALUES ($1) RETURNING *")
        .bind("hoge")
        .fetch_one(&mut **db_con)
        .await?;
    Ok(product)
}
