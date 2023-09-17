use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("hoge")]
pub struct Db(sqlx::PgPool);
