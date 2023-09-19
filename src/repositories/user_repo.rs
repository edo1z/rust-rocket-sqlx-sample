use crate::model::User;
use mockall::automock;
use sqlx::{pool::PoolConnection, query_as, Postgres, Transaction};

#[automock]
#[async_trait]
pub trait UserRepo {
    async fn find_all(&self, pool: &mut PoolConnection<Postgres>)
        -> Result<Vec<User>, sqlx::Error>;
    async fn create_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<User, sqlx::Error>;
}

pub struct UserRepoImpl;
impl UserRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_all(
        &self,
        pool: &mut PoolConnection<Postgres>,
    ) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    async fn create_with_transaction<'a>(
        &self,
        txn: &mut Transaction<'a, Postgres>,
    ) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO users(name) VALUES ($1) RETURNING *",
            "hoge taro"
        )
        .fetch_one(txn)
        .await?;
        Ok(user)
    }
}
