use crate::db::DbCon;
use crate::error::app_error::AppError;
use crate::models::user_model::User;
use crate::repositories::error::DbRepoError;
use crate::repositories::repositories::Repos;
use mockall::automock;
use tracing::instrument;

pub struct UserUseCaseImpl {}
impl UserUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[automock]
#[async_trait]
pub trait UserUseCase: Send + Sync {
    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<User>, AppError>;
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        name: &String,
    ) -> Result<User, AppError>;
    async fn update(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        id: i32,
        name: &String,
    ) -> Result<User, AppError>;
    async fn delete(&self, repos: &Repos, db_con: &mut DbCon, id: i32) -> Result<(), AppError>;
}

#[async_trait]
impl UserUseCase for UserUseCaseImpl {
    #[instrument(name = "user_use_case/find_all", skip_all)]
    async fn find_all(&self, repos: &Repos, db_con: &mut DbCon) -> Result<Vec<User>, AppError> {
        repos
            .user
            .find_all(&mut *db_con)
            .await
            .map_err(|e| AppError::from(e))
    }

    #[instrument(name = "user_use_case/create", skip_all)]
    async fn create(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        name: &String,
    ) -> Result<User, AppError> {
        repos
            .user
            .create(&mut *db_con, name)
            .await
            .map_err(|e| AppError::from(e))
    }

    #[instrument(name = "user_use_case/update", skip_all, fields(id = %id))]
    async fn update(
        &self,
        repos: &Repos,
        db_con: &mut DbCon,
        id: i32,
        name: &String,
    ) -> Result<User, AppError> {
        match repos.user.update(&mut *db_con, id, name).await {
            Ok(user) => Ok(user),
            Err(e) => match &e {
                DbRepoError::SqlxError(sqlx_error) => match sqlx_error {
                    sqlx::Error::RowNotFound => Err(AppError::BadRequest),
                    _ => Err(AppError::from(e)),
                },
                _ => Err(AppError::from(e)),
            },
        }
    }

    #[instrument(name = "user_use_case/delete", skip_all, fields(id = %id))]
    async fn delete(&self, repos: &Repos, db_con: &mut DbCon, id: i32) -> Result<(), AppError> {
        repos
            .user
            .delete(&mut *db_con, id)
            .await
            .map_err(|e| AppError::from(e))
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
    async fn test_find_all() {
        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo
            .expect_find_all()
            .returning(|_| Ok(users_fixture(5)));
        let mut repos = create_repos_for_test();
        repos.user = Box::new(mock_user_repo);
        let mut db_con = create_db_con_for_test().await.unwrap();
        let user_use_case = UserUseCaseImpl::new();
        let users = user_use_case.find_all(&repos, &mut db_con).await;
        assert!(users.is_ok());
        assert_eq!(users.unwrap().len(), 5);
    }
}
