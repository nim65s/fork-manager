use fork_manager::Config;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    let input = File::open("fork-manager.yaml")?;
    let config: Config = serde_yml::from_reader(input)?;
    dbg!(config);
    Ok(())
}
