use std::path::PathBuf;

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
}

pub fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, "fork-manager", &mut std::io::stdout());
}
