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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::db::create_db_con_for_test;
    use crate::test::fixture::user::user_fixture;

    #[rocket::async_test]
    async fn test_find_all() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_repo = UserRepoImpl::new();
        let users = user_repo.find_all(&mut db_con).await;
        assert!(users.is_ok());
        assert_eq!(users.unwrap().len(), 1);
    }

    #[rocket::async_test]
    async fn test_create() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_repo = UserRepoImpl::new();
        let user = user_repo.create(&mut db_con).await;
        assert!(user.is_ok());
        assert_eq!(user.unwrap().name, user_fixture(1).name);
    }
}