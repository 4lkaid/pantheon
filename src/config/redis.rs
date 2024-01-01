use anyhow::Result;
use config::Config;

pub async fn init(config: &Config) -> Result<redis::Client> {
    let url = config.get_string("redis.url")?;
    let client = redis::Client::open(url)?;
    let _ = client.get_tokio_connection().await?;
    Ok(client)
}
