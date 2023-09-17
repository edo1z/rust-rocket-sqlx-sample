use crate::model::{Product, User};
use sqlx::{PgConnection, Postgres, Transaction};

pub async fn find_all(db_con: &mut PgConnection) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(db_con)
        .await?;
    Ok(products)
}

pub async fn create(db_con: &mut Transaction<'_, Postgres>) -> Result<Product, sqlx::Error> {
    let product =
        sqlx::query_as::<_, Product>("INSERT INTO products (name) VALUES ($1) RETURNING *")
            .bind("hoge")
            .fetch_one(db_con)
            .await?;
    Ok(product)
}

pub async fn create_user(db_con: &mut Transaction<'_, Postgres>) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("INSERT INTO users(name) VALUES ($1) RETURNING *")
        .bind("hoge taro")
        .fetch_one(db_con)
        .await?;
    Ok(user)
}
