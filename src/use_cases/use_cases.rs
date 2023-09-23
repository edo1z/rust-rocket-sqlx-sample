use crate::use_cases::{
    product_use_case::{ProductUseCase, ProductUseCaseImpl},
    user_use_case::{UserUseCase, UserUseCaseImpl},
};

pub struct UseCases {
    pub user: Box<dyn UserUseCase>,
    pub product: Box<dyn ProductUseCase>,
}

pub fn create_use_cases() -> UseCases {
    let user = Box::new(UserUseCaseImpl::new());
    let product = Box::new(ProductUseCaseImpl::new());
    UseCases { user, product }
}
