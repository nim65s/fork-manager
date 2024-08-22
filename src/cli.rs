use super::{Error, Result};
use clap::CommandFactory;
use std::path::{Path, PathBuf};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file.
    /// If not given, or not a file, this will be searched
    /// according to arguments "project" and "filename"
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_CONFIG_FILE",
        default_value = "./fork-manager.yaml",
        value_hint = clap::ValueHint::FilePath,
    )]
    pub config_file: PathBuf,

    /// Name of the configuration file to look for
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_CONFIG_FILENAME",
        default_value = "fork-manager.yaml",
        value_hint = clap::ValueHint::FilePath,
    )]
    pub filename: PathBuf,

    /// Path to the project where to look for
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_PROJECT",
        default_value = ".",
        value_hint = clap::ValueHint::DirPath
    )]
    pub project: PathBuf,

    /// If provided, outputs the completion file for given shell and exit
    #[arg(long = "generate", value_enum)]
    pub generator: Option<clap_complete::Shell>,

    /// Only check config, don't run git commands
    #[arg(short, long)]
    pub dry_run: bool,

    /// Really force push: deactivate dry run.
    #[arg(long)]
    pub push: bool,
}

impl Args {
    pub fn process(&mut self) -> Result<bool> {
        if let Some(generator) = self.generator {
            let mut cmd = Args::command();
            print_completions(generator, &mut cmd);
            Ok(false)
        } else {
            if !self.config_file.is_file() {
                self.config_file = find_config_file(&self.project, &self.filename)?;
            }

            // now that we have a proper canonical config_file, we can ensure project root
            for ancestor in self.config_file.ancestors() {
                if self.config_file == ancestor.join(&self.filename) {
                    self.project = ancestor.to_path_buf();
                }
            }
            Ok(true)
        }
    }
}

pub fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, "fork-manager", &mut std::io::stdout());
}

fn find_config_file(project: &Path, filename: &Path) -> Result<PathBuf> {
    let mut dir = project.canonicalize()?;
    let mut path;
    loop {
        path = dir.join(filename);
        if path.is_file() {
            return Ok(path);
        }
        dir = dir
            .parent()
            .ok_or(Error::NotFound(project.to_path_buf()))?
            .to_path_buf();
    }
}
