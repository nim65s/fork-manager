use std::fs::File;

mod cli;
mod config;
mod error;
mod template;

pub use cli::{print_completions, Args};
pub use config::{Change, Config, Fork, Repo, Update, PR};
pub use error::{Error, Result};
pub use template::generate;

pub struct ForkManager {
    args: Args,
    config: Config,
}

impl ForkManager {
    pub async fn new(args: Args) -> Result<Self> {
        let config_file = File::open(&args.config_file)?;
        let mut config: Config = serde_yml::from_reader(config_file)?;
        config.update().await?;
        Ok(Self { args, config })
    }

    pub async fn main(&mut self) -> Result<()> {
        if self.args.dry_run {
            dbg!(&self.config);
        } else {
            generate(self)?;
        }
        Ok(())
    }
}
