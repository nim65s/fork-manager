use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};

mod cli;
mod error;

pub use cli::{print_completions, Args};
pub use error::{Error, Result};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Repo {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Change {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
        let url = pr
            .head
            .repo
            .ok_or(Error::GithubParseError("Missing repo head".to_string()))?
            .html_url
            .ok_or(Error::GithubParseError("Missing repo html url".to_string()))?
            .to_string();
        let branch = pr.head.ref_field;
        let title = Some(pr.title.unwrap_or(branch.clone()));
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
        let re = regex::Regex::new(r"github.com[/:]([^/]+)/([^/]+)")?;
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

    pub fn fill(&mut self) {
        // When upstream branch is not provided, we can use target branch
        if self.upstream.branch.is_none() {
            if let Some(branch) = &self.target.branch {
                self.upstream.branch = Some(branch.clone());
            }
        }
        // when change title is not provided, we can use branch name
        for item in &mut self.changes {
            if let Update::Change(change) = item {
                if let Change {
                    title: None,
                    url: _,
                    branch,
                } = change
                {
                    change.title = Some(branch.to_string());
                }
            }
        }
    }

    pub async fn process(&mut self, _config: &Option<Repo>) -> Result<()> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Repo>,
    pub forks: Vec<Fork>,
}

impl Config {
    pub async fn new(args: &Args) -> Result<Self> {
        let config_file = File::open(&args.config_file)?;
        let mut config: Self = serde_yml::from_reader(config_file)?;
        config.update().await?;
        Ok(config)
    }

    pub async fn update(&mut self) -> Result<()> {
        for fork in &mut self.forks {
            fork.get_prs().await?;
            fork.fill();
        }
        Ok(())
    }

    pub async fn process(&mut self, args: &Args) -> Result<()> {
        for fork in &mut self.forks {
            fork.process(&self.config).await?;
        }
        Ok(())
    }
}
