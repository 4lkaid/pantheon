pub mod database;
pub mod logger;
pub mod redis;

pub type Config = ::config::Config;

use anyhow::Result;

pub fn init() -> Result<Config> {
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()?;
    Ok(config)
}
