use axum::Json;
use crate::models::PullRequestEvent;


pub async fn webhook(Json(payload): Json<PullRequestEvent>){
    println!("PR action: {}", payload.action);
}
