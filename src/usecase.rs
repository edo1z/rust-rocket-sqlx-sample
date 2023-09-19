use crate::app_state::AppState;
use sqlx::pool::PoolConnection;
use sqlx::Acquire;
use sqlx::Postgres;
use mockall::automock;

#[automock]
#[async_trait]
pub trait UseCase {
    async fn get_all_products_and_users(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState
    ) -> Result<String, sqlx::Error>;
    async fn create_product_and_user(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState
    ) -> Result<String, sqlx::Error>;
}

pub struct UseCaseImpl;
impl UseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UseCase for UseCaseImpl {
    async fn get_all_products_and_users(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState
    ) -> Result<String, sqlx::Error> {
        let products = app.repo.find_all_products(&mut *pool).await?;
        let users = app.repo.find_all_users(&mut *pool).await?;
        Ok(format!("products: {:?} usres: {:?}", products, users))
    }

    async fn create_product_and_user(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState
    ) -> Result<String, sqlx::Error> {
        let mut txn = pool.begin().await?;

        let user = app.repo.create_user_with_transaction(&mut txn).await?;
        let product = app.repo.create_with_transaction(&mut txn).await?;

        // txn.rollback().await.unwrap();
        txn.commit().await.unwrap();

        Ok(format!("product:{:?}, user:{:?}", product, user))
    }
}
