use crate::repositories::repositories::{create_repos, Repos};
use crate::use_cases::use_cases::{create_use_cases, UseCases};
use rocket::State;
use std::sync::Arc;

pub type AppState = State<Arc<App>>;

pub struct App {
    pub use_cases: UseCases,
    pub repos: Repos,
}

impl App {
    pub fn new(use_cases: UseCases, repos: Repos) -> Self {
        Self { use_cases, repos }
    }
}

pub fn create_app() -> Arc<App> {
    let repos = create_repos();
    let use_cases = create_use_cases();
    Arc::new(App::new(use_cases, repos))
}
