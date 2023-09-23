use crate::db::DbCon;
use crate::repositories::repositories::Repos;
use mockall::automock;

pub struct UserUseCaseImpl {}
impl UserUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserUseCase: Send + Sync {
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error>;
    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error>;
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error> {
        let users = repos.user.find_all(&mut *db_con).await?;
        Ok(format!("usres: {:?}", users))
    }

    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<String, sqlx::Error> {
        let user = repos.user.create(&mut *db_con).await?;
        Ok(format!("user:{:?}", user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::app::create_app_for_test;
    use rocket_db_pools::Database;

    #[rocket::async_test]
    async fn test_get_all() {
        let mut app_state = create_app_for_test();
        // let mut mock_pool: PoolConnection<Postgres> =
    }
}
