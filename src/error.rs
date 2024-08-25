#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("serde yml error: {0}")]
    SerdeYml(#[from] serde_yml::Error),

    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("octocrab error: {0}")]
    Octocrab(#[from] octocrab::Error),

    #[error("minijinja error: {0}")]
    MiniJinja(#[from] minijinja::Error),

    #[error("Can't parse owner/repo from this github url: {0}")]
    GithubParseError(String),

    #[error("Can't find fork-manager.yaml in {0}")]
    NotFound(std::path::PathBuf),
}

pub type Result<T> = core::result::Result<T, Error>;
