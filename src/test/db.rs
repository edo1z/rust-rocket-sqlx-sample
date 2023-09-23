use dotenv::dotenv;
use std::env;
use sqlx::PgPool;

async fn generate_pool() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(db_url.as_str())
        .await
        .unwrap();
    pool
}
