use std::process::Command;
use std::fs;

pub fn checkout_repo(
    owner: &str,
    repo: &str,
    sha: &str,
) -> Result<String, Box<dyn std::error::Error>>{
    let workspace = format!("/tmp/pr-automation-{}", sha);


    fs::create_dir_all(&workspace)?;

    let repo_url = format!("https://github.com/{}/{}.git", owner, repo);

    let status = Command::new("git")
        .args([
            "clone",
            &repo_url,
            &workspace
        ])
        .status()?;

    if !status.success(){
        return Err("git clone failed".into());
    }

    let status = Command::new("git")
        .args([
            "-C",
            &workspace,
            "checkout",
            sha
        ])
        .status()?;

    if !status.success(){
        return Err("git checkout failed".into());
    }

    let output = Command::new("git")
        .args([
            "-C",
            &workspace,
            "rev-parse",
            "HEAD",
        ])
        .output()?;

    let checked_out_sha = String::from_utf8(output.stdout)?;

    println!("Checked out SHA: {}",checked_out_sha.trim());

    Ok(workspace)

}
