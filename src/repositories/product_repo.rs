use crate::model::Product;
use mockall::automock;
use sqlx::{pool::PoolConnection, query_as, Postgres, Transaction};

#[automock]
#[async_trait]
pub trait ProductRepo {
    async fn find_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<Product>, sqlx::Error>;
    async fn create_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<Product, sqlx::Error>;
}

pub struct ProductRepoImpl;
impl ProductRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn find_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let products = query_as!(Product, "SELECT * FROM products")
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
}
