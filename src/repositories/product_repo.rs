use crate::log_into;
use crate::models::product_model::Product;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;

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

#[cfg(test)]
mod tests {
    use crate::repositories::product_repo::{ProductRepo, ProductRepoImpl};
    use crate::test::db::create_db_con_for_test;
    use crate::test::repositories::prepare::product::create_product;
    use sqlx::Connection;

    #[tokio::test]
    async fn test_create_product() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let result = create_product(&mut tx).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_product_by_id() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let product = create_product(&mut tx).await.unwrap();
        let repo = ProductRepoImpl::new();
        let result = repo.find_by_id(&mut tx, product.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_update_product() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let product = create_product(&mut tx).await.unwrap();
        let repo = ProductRepoImpl::new();
        let new_name = "new_name".to_string();
        let result = repo.update(&mut tx, product.id, &new_name).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_product() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let mut tx = db_con.begin().await.unwrap();
        let product = create_product(&mut tx).await.unwrap();
        let repo = ProductRepoImpl::new();
        let result = repo.delete(&mut tx, product.id).await;
        assert!(result.is_ok());
        tx.rollback().await.unwrap();
    }
}
