use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("db")]
pub struct DbConnection(sqlx::PgPool);
