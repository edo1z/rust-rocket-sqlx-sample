use crate::log_into;
use crate::models::user::User;
use crate::repositories::error::DbRepoError;
use mockall::automock;
use sqlx::{query, query_as, PgConnection};
use tracing::instrument;

pub struct UserRepoImpl {}

impl UserRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self, con: &mut PgConnection, name: &String) -> Result<User, DbRepoError>;
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<User>, DbRepoError>;
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<User>, DbRepoError>;
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        name: &String,
    ) -> Result<User, DbRepoError>;
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    #[instrument(name = "user_repo/create", skip_all)]
    async fn create(&self, con: &mut PgConnection, name: &String) -> Result<User, DbRepoError> {
        query_as!(
            User,
            "INSERT INTO users (name) VALUES ($1) RETURNING *",
            name,
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/find_all", skip_all)]
    async fn find_all(&self, con: &mut PgConnection) -> Result<Vec<User>, DbRepoError> {
        let users = query_as!(User, "SELECT * FROM users")
            .fetch_all(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(users)
    }

    #[instrument(name = "user_repo/find_by_id", skip_all, fields(id = %id))]
    async fn find_by_id(
        &self,
        con: &mut PgConnection,
        id: i32,
    ) -> Result<Option<User>, DbRepoError> {
        query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/update", skip_all, fields(id = %id))]
    async fn update(
        &self,
        con: &mut PgConnection,
        id: i32,
        name: &String,
    ) -> Result<User, DbRepoError> {
        query_as!(
            User,
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING *",
            name,
            id
        )
        .fetch_one(&mut *con)
        .await
        .map_err(|e| log_into!(e, DbRepoError))
    }

    #[instrument(name = "user_repo/delete", skip_all, fields(id = %id))]
    async fn delete(&self, con: &mut PgConnection, id: i32) -> Result<(), DbRepoError> {
        query!("DELETE FROM users WHERE id = $1", id)
            .execute(&mut *con)
            .await
            .map_err(|e| log_into!(e, DbRepoError))?;
        Ok(())
    }
}
