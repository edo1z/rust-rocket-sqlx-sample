use crate::app_err;
use crate::db::DbCon;
use crate::error::app_error::AppError;
use crate::models::product::Product;
use crate::repositories::repositories::Repos;
use mockall::automock;

pub struct ProductUseCaseImpl {}

impl ProductUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait ProductUseCase: Send + Sync {
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        name: &String,
    ) -> Result<Product, AppError>;
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Product>, AppError>;
}

#[async_trait]
impl ProductUseCase for ProductUseCaseImpl {
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        name: &String,
    ) -> Result<Product, AppError> {
        let product = repos.product.create(&mut *db_con, name).await?;
        Ok(product)
    }

    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<Product>, AppError> {
        let products = repos.product.find_all(&mut *db_con).await?;
        Ok(products)
    }
}
