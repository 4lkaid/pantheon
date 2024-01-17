use crate::common::error::Error;
use anyhow::Result;
use config::Config;
use std::sync::OnceLock;

static REDIS: OnceLock<redis::Client> = OnceLock::new();

pub async fn init(config: &Config) -> Result<()> {
    let url = config.get_string("redis.url")?;
    let client = redis::Client::open(url)?;
    let _ = client.get_tokio_connection().await?;
    let _ = REDIS.set(client);
    Ok(())
}

pub async fn conn() -> Result<redis::aio::Connection> {
    Ok(REDIS
        .get()
        .unwrap_or_else(|| panic!("redis client not initialized"))
        .get_tokio_connection()
        .await
        .map_err(|err| Error::Redis(err))?)
}
