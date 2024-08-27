use clap::Parser;
use fork_manager::{Args, ForkManager, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();
    if args.process()? {
        let mut fm = ForkManager::new(args).await?;
        fm.main().await
    } else {
        Ok(())
    }
}
