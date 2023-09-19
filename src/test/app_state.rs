#[cfg(test)]
use crate::app_state::{AppState, Repos, UseCases};
#[cfg(test)]
use crate::repositories::{product_repo, user_repo};
#[cfg(test)]
use crate::use_cases::{product_use_case, user_use_case};
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
pub fn create_app_state_for_test() -> AppState {
    let use_cases = UseCases {
        user: Box::new(user_use_case::MockUserUseCase::new()),
        product: Box::new(product_use_case::MockProductUseCase::new()),
    };
    let repos = Repos {
        user: Arc::new(user_repo::MockUserRepo::new()),
        product: Arc::new(product_repo::MockProductRepo::new()),
    };
    AppState {
        use_case: use_cases,
        repo: repos,
    }
}
