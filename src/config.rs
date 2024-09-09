use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{Error, Result};

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
        let mut octo = octocrab::Octocrab::builder();
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            octo = octo.personal_token(token)
        }
        let pr = octo.build()?.pulls(owner, repo).get(self.pr).await?;
        let mut url = pr
            .head
            .repo
            .ok_or(Error::GithubParseError("Missing repo head".to_string()))?
            .ssh_url
            .ok_or(Error::GithubParseError("Missing repo html url".to_string()))?
            .to_string();
        if let Some(strip) = url.strip_suffix(".git") {
            url = strip.to_string();
        }
        let branch = pr.head.ref_field;
        let title = Some(pr.title.unwrap_or(branch.clone()));
        if let Some(octocrab::models::IssueState::Closed) = pr.state {
            if let Some(url) = pr.html_url {
                eprintln!("⚠️ This PR is closed: {}", url.as_str());
            }
        }
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
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Repo>,
    pub forks: Vec<Fork>,
}

impl Config {
    pub async fn update(&mut self) -> Result<()> {
        for fork in &mut self.forks {
            fork.get_prs().await?;
            fork.fill();
        }
        Ok(())
    }

    pub fn remotes(&self) -> String {
        let mut remotes = HashSet::new();
        for fork in &self.forks {
            remotes.insert(fork.target.url.clone());
            remotes.insert(fork.upstream.url.clone());
            for update in &fork.changes {
                if let Update::Change(change) = update {
                    remotes.insert(change.url.clone());
                }
            }
        }
        let mut vec = remotes.into_iter().collect::<Vec<String>>();
        vec.sort();
        vec.join(" ")
    }
}
