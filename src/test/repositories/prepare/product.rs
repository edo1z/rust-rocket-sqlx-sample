use crate::models::product_model::Product;
use crate::repositories::error::DbRepoError;
use crate::repositories::product_repo::{ProductRepo, ProductRepoImpl};
use sqlx::postgres::PgConnection;

pub async fn create_product(db_con: &mut PgConnection) -> Result<Product, DbRepoError> {
    let product_repo = ProductRepoImpl::new();
    let name = "新しいプロジェクト".to_string();
    product_repo.create(&mut *db_con, &name).await
}
