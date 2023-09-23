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
    async fn truncate(&self, con: &mut DbCon) -> Result<(), sqlx::Error>;
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

    async fn truncate(&self, con: &mut DbCon) -> Result<(), sqlx::Error> {
        sqlx::query!("TRUNCATE users RESTART IDENTITY").execute(&mut **con).await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::db::create_db_con_for_test;

    #[rocket::async_test]
    async fn test_find_all() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_repo = UserRepoImpl::new();

        // save 3user
        user_repo.truncate(&mut db_con).await.unwrap();
        for _ in 0..3 { user_repo.create(&mut db_con).await.unwrap(); }

        // test
        let users = user_repo.find_all(&mut db_con).await;
        assert!(users.is_ok());
        assert_eq!(users.unwrap().len(), 3);
    }

    #[rocket::async_test]
    async fn test_create() {
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_repo = UserRepoImpl::new();

        user_repo.truncate(&mut db_con).await.unwrap();

        // test
        let user = user_repo.create(&mut db_con).await;
        assert!(user.is_ok());
        assert_eq!(user.unwrap().name, "hoge taro");

    }
}
