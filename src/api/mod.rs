pub mod database;
pub mod v1;

use std::net::SocketAddr;

use axum::Router;
use sqlx::PgPool;

pub async fn run_api(pool: PgPool, bind_addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new().nest("/api/v1", v1::router(pool));
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
