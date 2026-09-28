use axum::Json;
use crate::{
    models::PullRequestEvent,
    github::get_pull_request,
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
    println!("Title: {}", pr.state);
    println!("Title: {}", pr.html_url);


}
