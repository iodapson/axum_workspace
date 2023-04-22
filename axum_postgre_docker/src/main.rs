mod routes;

use routes::build_routes;
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

pub async fn run_app() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = build_routes();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8090));

    tracing::debug!("listening on {}", addr);

    // run app  with hyper on localhost:8080
    axum::Server::bind(&addr) // alternate syntax: &"0.0.0.0:8080".parse().unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    //let spawn = tokio::spawn(async { run_app });
    //spawn.await.unwrap();
    run_app().await;
}
