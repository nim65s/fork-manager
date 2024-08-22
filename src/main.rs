use clap::Parser;

use fork_manager::{Args, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    if args.process()? {
        let config = Config::new(&args).await?;
        if !&args.dry_run {
            config.process(&args).await?;
        }
    }
    Ok(())
}
