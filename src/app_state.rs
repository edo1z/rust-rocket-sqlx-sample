use std::sync::Arc;
use crate::usecase;
use crate::repo;

pub struct AppState {
    pub usecase: Box<dyn usecase::UseCase + Send + Sync>,
    pub repo: Arc<dyn repo::Repo + Send + Sync>,
}

pub fn create_app_state() -> AppState {
    let usecase_impl = Box::new(usecase::UseCaseImpl::new());
    let repo_impl = Arc::new(repo::RepoImpl::new());
    AppState {
        usecase: usecase_impl,
        repo: repo_impl,
    }
}