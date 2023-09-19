use crate::repositories::{product_repo, user_repo};
use crate::use_cases::{product_use_case, user_use_case};
use std::sync::Arc;

pub struct AppState {
    pub use_case: UseCases,
    pub repo: Repos,
}

pub struct UseCases {
    pub user: Box<dyn user_use_case::UserUseCase + Send + Sync>,
    pub product: Box<dyn product_use_case::ProductUseCase + Send + Sync>,
}

pub struct Repos {
    pub user: Arc<dyn user_repo::UserRepo + Send + Sync>,
    pub product: Arc<dyn product_repo::ProductRepo + Send + Sync>,
}

pub fn create_app_state() -> AppState {
    let use_cases = UseCases {
        user: Box::new(user_use_case::UserUseCaseImpl::new()),
        product: Box::new(product_use_case::ProductUseCaseImpl::new()),
    };
    let repos = Repos {
        user: Arc::new(user_repo::UserRepoImpl::new()),
        product: Arc::new(product_repo::ProductRepoImpl::new()),
    };
    AppState {
        use_case: use_cases,
        repo: repos,
    }
}
