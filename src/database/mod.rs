pub mod models;
pub mod queries;

use sqlx::{Executor, PgPool, postgres::PgPoolOptions};

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect(database_url).await?;
    let migration = include_str!("../../migrations/001_init.sql");
    pool.execute(migration).await?;
    Ok(pool)
}
