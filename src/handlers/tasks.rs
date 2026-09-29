use std::fs;

use axum::Json;
use crate::{
    github::get_pull_request, models::PullRequestEvent, workspace::{checkout_repo, run_tests},
};


pub async fn webhook(Json(payload): Json<PullRequestEvent>){
    println!("Repository: {}", payload.repository.full_name);
    println!("Owner: {}", payload.repository.owner.login);
    println!("PR action: {}", payload.action);
    println!("PR: {}", payload.number);
    println!("Source: {}", payload.pull_request.head.r#ref);
    println!("Targer: {}", payload.pull_request.base.r#ref);
    println!("SHA: {}", payload.pull_request.head.sha);

    let pr = match get_pull_request(&payload.repository.owner.login, &payload.repository.name, payload.number)
        .await {
            Ok(pr) => pr,
            Err(err) => {
                eprintln!("GitHub API Error: {}", err);
                return;
            }
        };

    println!("Title: {}", pr.title);
    println!("state: {}", pr.state);
    println!("URL: {}", pr.html_url);
    println!("Branch: {}", pr.head.r#ref);
    println!("SHA: {}", pr.head.sha);

    let workspace = match checkout_repo(
        &payload.repository.owner.login,
        &payload.repository.name,
        &pr.head.sha
    ) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Checkout error: {}", err);
            return;
        }
    };

    println!("PR checkedout at: {}", workspace);

    let (tests_passed, output)= match run_tests(&workspace){
        Ok(result) => result,
        Err(err) => {
            eprintln!("Test execution error {}", err);
            return;
        }
    };

    println!("Test output: \n{}", output);

    if tests_passed{
        println!("Passed tests");
    } else {
        println!("test failed");
    }

    match fs::remove_dir_all(&workspace) {
        Ok(_) => println!("Workspace cleaned up"),
        Err(err) => eprintln!("Workspace cleanup error: {}", err),
    }

}
