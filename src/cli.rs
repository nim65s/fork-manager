use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the configuration file to parse
    #[arg(
        short,
        long,
        env = "FORK_MANAGER_FILENAME",
        default_value = "fork-manager.yaml"
    )]
    pub filename: PathBuf,

    /// Path to the project to process
    #[arg(short, long, env = "FORK_MANAGER_PROJECT", default_value = ".")]
    pub project: PathBuf,
}
