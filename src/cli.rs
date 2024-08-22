use super::{Error, Result};
use clap::{CommandFactory, Parser};
use std::path::{Path, PathBuf};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the configuration file to parse
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_FILENAME",
        default_value = "fork-manager.yaml",
        value_hint = clap::ValueHint::FilePath,
    )]
    pub filename: PathBuf,

    /// Path to the project to process
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_PROJECT",
        default_value = ".",
        value_hint = clap::ValueHint::DirPath
    )]
    pub project: PathBuf,

    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    pub generator: Option<clap_complete::Shell>,

    pub config: PathBuf,
}

impl Args {
    pub fn get() -> Result<Option<Self>> {
        let mut args = Args::parse();
        if let Some(generator) = args.generator {
            let mut cmd = Args::command();
            print_completions(generator, &mut cmd);
            Ok(None)
        } else {
            args.config = find_config_file(&args.project, &args.filename)?;
            Ok(Some(args))
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
