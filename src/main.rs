use clap::Parser;
use fork_manager::{Args, ForkManager, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut fm = ForkManager::new(args).await?;
    fm.main().await
}
