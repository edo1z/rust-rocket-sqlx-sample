use crate::repositories::{
    product_repo::{ProductRepo, ProductRepoImpl},
    user_repo::{UserRepo, UserRepoImpl},
};

pub struct Repos {
    pub user: Box<dyn UserRepo>,
    pub product: Box<dyn ProductRepo>,
}

pub fn create_repos() -> Repos {
    let user = Box::new(UserRepoImpl::new());
    let product = Box::new(ProductRepoImpl::new());
    Repos { user, product }
}
