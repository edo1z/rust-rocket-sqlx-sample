use crate::model::{Product, User};
use sqlx::{query_as, PgExecutor};

pub async fn find_all_products(executor: impl PgExecutor<'_>) -> Result<Vec<Product>, sqlx::Error> {
    let products = query_as!(Product, "SELECT * FROM products")
        .fetch_all(executor)
        .await?;
    Ok(products)
}

pub async fn find_all_users(executor: impl PgExecutor<'_>) -> Result<Vec<User>, sqlx::Error> {
    let products = query_as!(User, "SELECT * FROM users")
        .fetch_all(executor)
        .await?;
    Ok(products)
}

pub async fn create(executor: impl PgExecutor<'_>) -> Result<Product, sqlx::Error> {
    let product = query_as!(
        Product,
        "INSERT INTO products (name) VALUES ($1) RETURNING *",
        "hoge"
    )
    .fetch_one(executor)
    .await?;

    Ok(product)
}

pub async fn create_user(executor: impl PgExecutor<'_>) -> Result<User, sqlx::Error> {
    let user = query_as!(
        User,
        "INSERT INTO users(name) VALUES ($1) RETURNING *",
        "hoge taro"
    )
    .fetch_one(executor)
    .await?;
    Ok(user)
}
