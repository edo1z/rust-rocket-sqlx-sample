use crate::db::DbCon;
use crate::model::Product;
use mockall::automock;
use sqlx::query_as;

pub struct ProductRepoImpl {}
impl ProductRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn find_all(&self, con: &mut DbCon) -> Result<Vec<Product>, sqlx::Error>;
    async fn create(&self, con: &mut DbCon) -> Result<Product, sqlx::Error>;
}

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    async fn find_all(&self, con: &mut DbCon) -> Result<Vec<Product>, sqlx::Error> {
        let products = query_as!(Product, "SELECT * FROM products")
            .fetch_all(&mut **con)
            .await?;
        Ok(products)
    }

    async fn create(&self, con: &mut DbCon) -> Result<Product, sqlx::Error> {
        let product = query_as!(
            Product,
            "INSERT INTO products (name) VALUES ($1) RETURNING *",
            "hoge"
        )
        .fetch_one(&mut **con)
        .await?;

        Ok(product)
    }
}
