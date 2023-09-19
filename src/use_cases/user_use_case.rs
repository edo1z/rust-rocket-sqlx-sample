use crate::app_state::AppState;
use mockall::automock;
use sqlx::pool::PoolConnection;
use sqlx::Acquire;
use sqlx::Postgres;

#[automock]
#[async_trait]
pub trait UserUseCase {
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

pub struct UserUseCaseImpl;
impl UserUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn get_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error> {
        let users = app.repo.user.find_all(&mut *pool).await?;
        Ok(format!("usres: {:?}", users))
    }

    async fn create(
        &self,
        pool: &mut PoolConnection<Postgres>,
        app: &AppState,
    ) -> Result<String, sqlx::Error> {
        let mut txn = pool.begin().await?;

        let user = app.repo.user.create_with_transaction(&mut txn).await?;

        // txn.rollback().await.unwrap();
        txn.commit().await.unwrap();

        Ok(format!("user:{:?}", user))
    }
}
