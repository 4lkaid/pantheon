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

pub fn get() -> &'static redis::Client {
    REDIS
        .get()
        .unwrap_or_else(|| panic!("redis client not initialized"))
}
