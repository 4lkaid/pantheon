pub mod common;
pub mod config;
pub mod handler;
pub mod middleware;
pub mod route;

use anyhow::{Context, Result};
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;
use tracing_appender::non_blocking::WorkerGuard;

pub struct AppState {
    pub db: PgPool,
    pub redis: Client,
}

pub type AppResult<T> = Result<T, common::error::Error>;

async fn serve(config: config::Config, state: Arc<AppState>) -> Result<()> {
    let router = route::api::init(state);
    let listener = tokio::net::TcpListener::bind(
        config
            .get_string("general.listen")
            .unwrap_or("0.0.0.0:8000".to_string()),
    )
    .await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await?;
    Ok(())
}

pub async fn run() -> Result<WorkerGuard> {
    let config = config::init().with_context(|| "configuration parsing failed")?;
    let pg_pool = config::database::init(&config)
        .await
        .with_context(|| "database connection failed")?;
    let redis_client = config::redis::init(&config)
        .await
        .with_context(|| "redis connection failed")?;
    let worker_guard = config::logger::init(&config);
    let state = Arc::new(AppState {
        db: pg_pool,
        redis: redis_client,
    });
    serve(config, state)
        .await
        .with_context(|| "service startup failed")?;
    Ok(worker_guard)
}
