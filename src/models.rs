use serde::Deserialize;


#[derive(Debug)]
pub enum ProjectType {
    Rust,
    Node,
    Python,
}

#[derive(Debug, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
    pub number: u64,
    pub repository: Repository,
    pub pull_request: PullRequest,
}

#[derive(Debug, Deserialize)]
pub struct PullRequest{
    pub head: Branch,
    pub base: Branch,
}

#[derive(Debug, Deserialize)]
pub struct Branch{
    pub r#ref: String,
    pub sha: String, 
}

#[derive(Debug, Deserialize)]
pub struct Repository{
    pub name: String,
    pub full_name: String,
    pub owner: Owner,
}

#[derive(Debug, Deserialize)]
pub struct Owner{
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubPullRequest {
    pub title: String,
    pub state: String, 
    pub html_url: String, 
    pub head: Branch,
}
