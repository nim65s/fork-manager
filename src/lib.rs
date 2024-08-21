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
    pub pr: usize,
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
