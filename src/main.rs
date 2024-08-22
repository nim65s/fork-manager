use clap::{CommandFactory, Parser};
use fork_manager::{print_completions, Args, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Some(generator) = args.generator {
        let mut cmd = Args::command();
        print_completions(generator, &mut cmd);
    } else {
        let config = Config::new(&args).await?;
        dbg!(config);
    }
    Ok(())
}
