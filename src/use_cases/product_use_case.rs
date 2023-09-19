use crate::app_state::AppState;
use mockall::automock;
use sqlx::pool::PoolConnection;
use sqlx::Acquire;
use sqlx::Postgres;

#[automock]
#[async_trait]
pub trait ProductUseCase {
    async fn get_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error>;
    async fn create(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error>;
}

pub struct ProductUseCaseImpl;
impl ProductUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProductUseCase for ProductUseCaseImpl {
    async fn get_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error> {
        let products = app.repo.product.find_all(&mut *pool).await?;
        Ok(format!("products: {:?}", products))
    }

    async fn create(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error> {
        let mut txn = pool.begin().await?;

        let product = app.repo.product.create_with_transaction(&mut txn).await?;

        // txn.rollback().await.unwrap();
        txn.commit().await.unwrap();

        Ok(format!("product:{:?}", product))
    }
}
