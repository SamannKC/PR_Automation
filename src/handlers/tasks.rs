use std::fs;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use axum::{
    body::Bytes,
    http::HeaderMap,
};
use crate::{
    github::{get_pull_request, report_status}, 
    models::PullRequestEvent, 
    workspace::{checkout_repo},
    runner::run_project,
    project
};


pub async fn webhook(
    headers: HeaderMap,
    body: Bytes,
) {

    let signature = match headers.get("X-Hub-Signature-256") {
        Some(value) => value,
        None => {
            eprintln!("Missing GitHub signature");
            return;
        }
    };

    let signature = match signature.to_str() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid signature header");
            return;
        }
    };

    let signature = match signature.strip_prefix("sha256=") {
        Some(value) => value,
        None => {
            eprintln!("Invalid signature format");
            return;
        }
    };

    let signature = match hex::decode(signature) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid signature encoding");
            return;
        }
    };

    let secret = std::env::var("WEBHOOK_SECRET")
        .expect("WEBHOOK_SECRET is not set");

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("Invalid HMAC key");

    mac.update(&body);

    if mac.verify_slice(&signature).is_err() {
        eprintln!("Invalid GitHub webhook signature");
        return;
    }

    let payload: PullRequestEvent = match serde_json::from_slice(&body) {
        Ok(payload) => payload,
        Err(err) => {
            eprintln!("Invalid webhook payload: {}", err);
            return;
        }
    };

    match payload.action.as_str() {
        "opened" | "synchronize" | "reopened" => {}
        _ => {
            println!("Ignoring PR action: {}", payload.action);
            return;
        }
    }

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

    let projects = project::detect_projects(&workspace);

    if projects.is_empty() {
        eprintln!("No supported projects detected");

        match report_status(
            &payload.repository.owner.login,
            &payload.repository.name,
            &pr.head.sha,
            "failure",
            "No supported projects detected",
        )
        .await
        {
            Ok(_) => println!("GitHub status reported"),
            Err(err) => eprintln!("Failed to report GitHub status: {}", err),
        }

        match fs::remove_dir_all(&workspace) {
            Ok(_) => println!("Workspace cleaned up"),
            Err(err) => eprintln!("Workspace cleanup error: {}", err),
        }

        return;
    }

    let mut all_passed = true;
    let mut results = Vec::new();

    for (project_type, path) in projects {
        let tests_passed= match run_project(&project_type, &path) {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Test execution error: {}", err);

                match fs::remove_dir_all(&workspace) {
                    Ok(_) => println!("Workspace cleaned up"),
                    Err(cleanup_err) => {
                        eprintln!("Workspace cleanup error: {}", cleanup_err);
                    }
                }

                return;
            }
        };

        if tests_passed {
            results.push(format!("{:?}: PASSED",project_type));
            println!("Passed tests");
        } else {
            results.push(format!("{:?}: FAILED",project_type));
            all_passed = false;
            println!("Tests failed");
        }
    }

    let description = results.join("  |  ");

    let state = if all_passed {
        "success"
    } else {
        "failure"
    };

    match report_status(
        &payload.repository.owner.login,
        &payload.repository.name,
        &pr.head.sha,
        state,
        &description,
    )
    .await
    {
        Ok(_) => println!("GitHub status reported"),
        Err(err) => eprintln!("Failed to report GitHub status: {}", err),
    }

    match fs::remove_dir_all(&workspace) {
        Ok(_) => println!("Workspace cleaned up"),
        Err(err) => eprintln!("Workspace cleanup error: {}", err),
    }

}
