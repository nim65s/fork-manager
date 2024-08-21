#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = fork_manager::Config::new().await?;
    dbg!(config);
    Ok(())
}
