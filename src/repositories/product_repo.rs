use crate::models::product::Product;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;
use crate::log_into;

pub struct ProductRepoImpl {}
impl ProductRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn create(&self, con: &mut PgConnection, name: &String) -> Result<Product, DbRepoError>;
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Product>, DbRepoError>;
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Product>, DbRepoError>;
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        name: &String,
    ) -> Result<Product, DbRepoError>;
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError>;
}

#[async_trait]
impl ProductRepo for ProductRepoImpl {
    #[instrument(name = "product_repo/create", skip_all)]
    async fn create(&self, con: &mut PgConnection, name: &String) -> Result<Product, DbRepoError> {
        query_as!(
            Product,
            "INSERT INTO products (name) VALUES ($1) RETURNING *",
            name,
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "product_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<Product>, DbRepoError> {
        let products = query_as!(Product, "SELECT * FROM products")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(products)
    }

    #[instrument(name = "product_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<Product>, DbRepoError> {
        query_as!(Product, "SELECT * FROM products WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "product_repo/update", skip_all, fields(id = %id))]
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        name: &String,
    ) -> Result<Product, DbRepoError> {
        query_as!(
            Product,
            "UPDATE products SET name = $1 WHERE id = $2 RETURNING *",
            name,
            id
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "product_repo/delete", skip_all, fields(id = %id))]
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError> {
        query!("DELETE FROM products WHERE id = $1", id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(())
    }
}
