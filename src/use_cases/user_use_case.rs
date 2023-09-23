use crate::db::DbCon;
use crate::repositories::repositories::Repos;
use mockall::automock;
use crate::model::User;

pub struct UserUseCaseImpl {}
impl UserUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserUseCase: Send + Sync {
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<User>, sqlx::Error>;
    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    async fn get_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<User>, sqlx::Error> {
        repos.user.find_all(&mut *db_con).await
    }

    async fn create(&self, repos: &Repos, db_con: &mut DbCon) -> Result<User, sqlx::Error> {
        repos.user.create(&mut *db_con).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_repo::MockUserRepo;
    use crate::test::app::create_repos_for_test;
    use crate::test::db::create_db_con_for_test;
    use crate::test::fixture::user::users_fixture;

    #[rocket::async_test]
    async fn test_get_all() {
        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo
            .expect_find_all()
            .returning(|_| Ok(users_fixture(5)));
        let mut repos = create_repos_for_test();
        repos.user = Box::new(mock_user_repo);
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_use_case = UserUseCaseImpl::new();
        let users = user_use_case.get_all(&repos, &mut db_con).await;
        assert!(users.is_ok());
        assert_eq!(users.unwrap().len(), 5);
    }
}
