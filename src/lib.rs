pub mod common;
pub mod config;
pub mod handler;
pub mod middleware;
pub mod route;

use anyhow::{Context, Result};
use std::net::SocketAddr;
use tracing_appender::non_blocking::WorkerGuard;

pub type AppResult<T> = Result<T, common::error::Error>;

async fn serve(config: config::Config) -> Result<()> {
    let router = route::api::init();
    let listener = tokio::net::TcpListener::bind(
        config
            .get_string("general.listen")
            .unwrap_or("0.0.0.0:8000".to_string()),
    )
    .await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

pub async fn run() -> Result<WorkerGuard> {
    let config = config::init().with_context(|| "configuration parsing failed")?;
    config::database::init(&config)
        .await
        .with_context(|| "database connection failed")?;
    config::redis::init(&config)
        .await
        .with_context(|| "redis connection failed")?;
    let worker_guard = config::logger::init(&config);
    serve(config)
        .await
        .with_context(|| "service startup failed")?;
    Ok(worker_guard)
}
