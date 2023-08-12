mod config;
mod http; // DIR
mod run_database; // Sibling-Module

use anyhow::{Context, Result};
use config::Config;
use http::build_routes;
use run_database::run_database;
use std::error::Error; // not strictly required here since anyhow is utilized.
use std::net::SocketAddr;
//use tracing;
//use tracing_subscriber;

pub async fn start_app() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = Config::new();

    let db = run_database(&config.database_url).await.unwrap();

    // -------- SERVER INIT START
    // Fn http::build_routes()' inside 'http/mod.rs' is my own little alternative to 'http::serve(config, db)' from Realworld API
    let routes_aggregate = build_routes(config, db);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8090));

    // run axum server on localhost:8090
    axum::Server::bind(&socket_addr)
        .serve(routes_aggregate.into_make_service())
        .await
        .with_context(|| "Server failed to start/serve")?;

    tracing::debug!(
        "This axum server started successfully and is now listening on {}",
        socket_addr
    );
    // -------- SERVER INIT END

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_app().await.unwrap();
    Ok(())
}
