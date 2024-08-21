use fork_manager::Config;
use std::fs::File;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let input = File::open("fork-manager.yaml")?;
    let mut config: Config = serde_yml::from_reader(input)?;
    //dbg!(config);
    config.get_prs().await?;
    dbg!(config);
    Ok(())
}
