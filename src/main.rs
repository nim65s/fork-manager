use clap::Parser;
use fork_manager::{Args, ForkManager, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut fm = ForkManager::new(Args::parse()).await?;
    fm.main().await
}
