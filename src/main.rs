use fork_manager::{Args, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Some(args) = Args::get()? {
        let config = Config::new(&args).await?;
        dbg!(config);
    }
    Ok(())
}
