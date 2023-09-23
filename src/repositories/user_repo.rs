use crate::db::DbCon;
use crate::model::User;
use mockall::automock;
use sqlx::query_as;

pub struct UserRepoImpl {}

impl UserRepoImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_all(&self, con: &mut DbCon) -> Result<Vec<User>, sqlx::Error>;
    async fn create(&self, con: &mut DbCon) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_all(&self, con: &mut DbCon) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(User, "SELECT * FROM users")
            .fetch_all(&mut **con)
            .await?;
        Ok(users)
    }

    async fn create(&self, con: &mut DbCon) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            "INSERT INTO users(name) VALUES ($1) RETURNING *",
            "hoge taro"
        )
        .fetch_one(&mut **con)
        .await?;
        Ok(user)
    }
}
