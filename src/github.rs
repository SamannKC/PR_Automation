use reqwest::Client;
use crate::models::GitHubPullRequest;

pub async fn get_pull_request(
    owner: &str,
    repo: &str,
    number: u64,
) -> Result<GitHubPullRequest, reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        owner, repo, number
    );

    let client = Client::new();

    let token = std::env::var("GITHUB_TOKEN")
        .expect("GitHub Token not set.");

    let response = client
        .get(url)
        .header("User-Agent", "pr-automation")
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;

    let pull_request = response
        .json::<GitHubPullRequest>()
        .await?;

    Ok(pull_request)
}

