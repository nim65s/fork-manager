use clap::Parser;
use fork_manager::{Args, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::new(args).await?;
    dbg!(config);
    Ok(())
}
