use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let _worker_guard = pantheon::run().await?;
    Ok(())
}
