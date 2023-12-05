#[macro_use]
extern crate rocket;

pub mod app;
pub mod config;
pub mod db;

mod error {
    pub mod app_error;
    pub mod logging;
}

mod controllers {
    pub mod product_controller;
    pub mod user_controller;
}
mod use_cases {
    pub mod product_use_case;
    pub mod use_cases;
    pub mod user_use_case;
}
mod repositories {
    pub mod error;
    pub mod product_repo;
    pub mod repositories;
    pub mod user_repo;
}

mod models {
    pub mod product_model;
    pub mod user_model;
}

mod dto {
    pub mod product_dto;
    pub mod user_dto;
}

#[cfg(test)]
mod test {
    pub mod app;
    pub mod db;
    pub mod fixture {
        pub mod product;
        pub mod user;
    }

    pub mod repositories {
        pub mod prepare {
            pub mod product;
            pub mod user;
        }
    }
}

use crate::app::create_app;
use crate::config::Config;
use crate::controllers::{product_controller, user_controller};
use crate::db::Db;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::config::<Config>())
        .manage(create_app())
        .mount("/users", user_controller::routes())
        .mount("/products", product_controller::routes())
}
