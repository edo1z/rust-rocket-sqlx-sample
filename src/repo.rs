use crate::model::{Product, User};
use mockall::automock;
use sqlx::{pool::PoolConnection, query_as, Postgres, Transaction};

#[automock]
#[async_trait]
pub trait Repo {
    async fn find_all_products(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<Product>, sqlx::Error>;
    async fn find_all_users(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error>;
    async fn create_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<Product, sqlx::Error>;
    async fn create_user_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<User, sqlx::Error>;
}

pub struct RepoImpl;
impl RepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repo for RepoImpl {
    async fn find_all_products(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let products = query_as!(Product, "SELECT * FROM products")
            .fetch_all(pool)
            .await?;
        Ok(products)
    }

    async fn find_all_users(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        let products = query_as!(User, "SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(products)
    }

    async fn create_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<Product, sqlx::Error> {
        let product = query_as!(
            Product,
            "INSERT INTO products (name) VALUES ($1) RETURNING *",
            "hoge"
        )
        .fetch_one(txn)
        .await?;

        Ok(product)
    }

    async fn create_user_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO users(name) VALUES ($1) RETURNING *",
            "hoge taro"
        )
        .fetch_one(txn)
        .await?;
        Ok(user)
    }
}
