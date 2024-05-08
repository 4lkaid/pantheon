use anyhow::Result;
use config::Config;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
use std::{sync::OnceLock, time::Duration};

static DB: OnceLock<PgPool> = OnceLock::new();

pub async fn init(config: &Config) -> Result<()> {
    let url = config.get_string("database.url")?;
    let max_connections = config
        .get_int("database.max_connections")
        .unwrap_or(10)
        .try_into()?;
    let min_connections = config
        .get_int("database.min_connections")
        .unwrap_or(0)
        .try_into()?;
    let acquire_timeout = config
        .get_int("database.acquire_timeout")
        .unwrap_or(30)
        .try_into()?;
    let idle_timeout = config
        .get_int("database.idle_timeout")
        .unwrap_or(10 * 60)
        .try_into()?;
    let max_lifetime = config
        .get_int("database.max_lifetime")
        .unwrap_or(30 * 60)
        .try_into()?;
    let pool = PgPoolOptions::new()
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                conn.execute("SET TIME ZONE 'Asia/Shanghai';").await?;
                Ok(())
            })
        })
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .idle_timeout(Some(Duration::from_secs(idle_timeout)))
        .max_lifetime(Some(Duration::from_secs(max_lifetime)))
        .connect(&url)
        .await?;
    let _ = DB.set(pool);
    Ok(())
}

pub fn conn() -> &'static PgPool {
    DB.get()
        .unwrap_or_else(|| panic!("database is not initialized"))
}
