#[cfg(test)]
use crate::app::App;
#[cfg(test)]
use crate::repositories::{
    product_repo::MockProductRepo, repositories::Repos, user_repo::MockUserRepo,
};
#[cfg(test)]
use crate::use_cases::{
    product_use_case::MockProductUseCase, use_cases::UseCases, user_use_case::MockUserUseCase,
};

#[cfg(test)]
pub fn create_app_for_test() -> App {
    let repos = create_repos_for_test();
    let use_cases = create_use_cases_for_test();
    App::new(use_cases, repos)
}

#[cfg(test)]
pub fn create_repos_for_test() -> Repos {
    let user = Box::new(MockUserRepo::new());
    let product = Box::new(MockProductRepo::new());
    Repos { user, product }
}

#[cfg(test)]
pub fn create_use_cases_for_test() -> UseCases {
    let user = Box::new(MockUserUseCase::new());
    let product = Box::new(MockProductUseCase::new());
    UseCases { user, product }
}
