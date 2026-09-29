mod models;
mod routes;
mod handlers;
mod github;
mod workspace;
mod project;
mod runner;

#[tokio::main]
async fn main() {
   let app = routes::routes();

   let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
       .await
       .unwrap();

    println!("listening on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
