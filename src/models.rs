use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PullRequestEvent {
    pub action: String,
}
