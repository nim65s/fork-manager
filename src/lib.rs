mod cli;
mod error;
pub use cli::{print_completions, Args};
pub use error::{Error, Result};
use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Repo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Change {
    pub title: String,
    pub url: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct PR {
    pub pr: u64,
}

impl PR {
    pub async fn to_change(&self, owner: String, repo: String) -> Result<Change> {
        let pr = octocrab::instance().pulls(owner, repo).get(self.pr).await?;
        // TODO if state == "closed", we should be able dismiss it
        let title = pr
            .title
            .ok_or(Error::GithubParseError("Missing PR title".to_string()))?;
        let url = pr
            .head
            .repo
            .ok_or(Error::GithubParseError("Missing repo head".to_string()))?
            .html_url
            .ok_or(Error::GithubParseError("Missing repo html url".to_string()))?
            .to_string();
        let branch = pr.head.ref_field;
        Ok(Change { title, url, branch })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Update {
    Change(Change),
    PR(PR),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Fork {
    pub name: String,
    pub target: Repo,
    pub upstream: Repo,
    pub changes: Vec<Update>,
}

impl Fork {
    pub fn parse_github(&self) -> Result<(String, String)> {
        let re = regex::Regex::new(r"github.com/([^/]+)/([^/]+)")?;
        let caps = re
            .captures(&self.upstream.url)
            .ok_or(Error::GithubParseError(self.upstream.url.clone()))?;
        Ok((caps[1].to_string(), caps[2].to_string()))
    }

    pub async fn get_prs(&mut self) -> Result<()> {
        let (owner, repo) = self.parse_github()?;
        for item in &mut self.changes {
            if let Update::PR(pr) = item {
                *item = Update::Change(pr.to_change(owner.clone(), repo.clone()).await?);
            }
        }
        Ok(())
    }

    pub fn get_upstream_branch(&mut self) {
        if self.upstream.branch.is_none() {
            if let Some(branch) = &self.target.branch {
                self.upstream.branch = Some(branch.clone());
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Repo>,
    pub forks: Vec<Fork>,
}

impl Config {
    pub async fn new(args: Args) -> Result<Self> {
        let config = File::open(find_config_file(args)?)?;
        let mut config: Self = serde_yml::from_reader(config)?;
        config.update().await?;
        Ok(config)
    }

    pub async fn update(&mut self) -> Result<()> {
        for fork in &mut self.forks {
            fork.get_prs().await?;
            fork.get_upstream_branch();
        }
        Ok(())
    }
}

pub fn find_config_file(args: Args) -> Result<PathBuf> {
    let mut dir = args.project.canonicalize()?;
    let mut path;
    loop {
        path = dir.join(args.filename.clone());
        if path.is_file() {
            return Ok(path);
        }
        dir = dir
            .parent()
            .ok_or(Error::NotFound(args.project.clone()))?
            .to_path_buf();
    }
}
