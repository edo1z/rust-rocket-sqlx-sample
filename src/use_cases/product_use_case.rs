use crate::db::DbCon;
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
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error>;
    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error>;
}

#[async_trait]
impl ProductUseCase for ProductUseCaseImpl {
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error> {
        let products = repos.product.find_all(&mut *db_con).await?;
        Ok(format!("products: {:?}", products))
    }

    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error> {
        let product = repos.product.create(&mut *db_con).await?;
        Ok(format!("product:{:?}", product))
    }
}
