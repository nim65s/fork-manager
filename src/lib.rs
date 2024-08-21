mod error;
pub use error::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Upstream {
    pub repo: String,
    pub branch: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Change {
    pub title: String,
    pub repo: String,
    pub branch: String,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct PR {
    pub pr: u64,
}

impl PR {
    pub async fn to_change(&self, owner: String, repo: String) -> Result<Change> {
        println!("owner: {owner:?}, repo: {repo:?}, pr: {:?}", self.pr);
        let pr = octocrab::instance().pulls(owner, repo).get(self.pr).await?;
        // TODO if state == "closed", we should be able dismiss it
        let title = pr
            .title
            .ok_or(Error::GithubParseError("Missing PR title".to_string()))?;
        let repo = pr
            .head
            .repo
            .ok_or(Error::GithubParseError("Missing repo head".to_string()))?
            .html_url
            .ok_or(Error::GithubParseError("Missing repo html url".to_string()))?
            .to_string();
        let branch = pr.head.ref_field;
        Ok(Change {
            title,
            repo,
            branch,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Update {
    Change(Change),
    PR(PR),
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub repo: String,
    pub branch: String,
    pub upstream: Upstream,
    pub changes: Vec<Update>,
}

impl Config {
    pub fn parse_github(&self) -> Result<(String, String)> {
        let re = regex::Regex::new(r"github.com/([^/]+)/([^/]+)")?;
        let caps = re
            .captures(&self.upstream.repo)
            .ok_or(Error::GithubParseError(self.upstream.repo.clone()))?;
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
}
