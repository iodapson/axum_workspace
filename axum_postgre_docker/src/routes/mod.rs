mod create_one_row;
mod data_sharing;
mod read_one_row;
mod return_json;
mod run_database;

use axum::Extension;
use axum::{routing::get, routing::post, Router};
use create_one_row::create_one_row;
use data_sharing::access_shared_data;
use dotenvy::dotenv;
use read_one_row::read_one_row;
use return_json::return_json_data;
use run_database::run_database;

#[derive(Clone)]
pub struct SharedData {
    pub data_one: String,
}

pub fn build_routes() -> Router {
    // instantiate (initialize) SharedData
    let shared_data = SharedData {
        data_one: "I am shared data one (1)".to_owned(),
    };

    // For working with .env which has the Postgres DB instance connection url
    dotenv().ok();

    // extract database url into a variable
    let database_url = std::env::var("DATABASE_URL").unwrap();
    dbg!("Server running...");

    // Now call the function that perfoms database operations
    let _database_ran_ok = run_database(database_url);

    // build our application with a single route for the root-path of our application
    Router::new()
        .route("/access_shared_data", get(access_shared_data)) // * route of concern
        .layer(Extension(shared_data)) // 'shared_data would be shared with route 'access_shared_data' only because it comes after before the layer
        .route("/return-json-data", get(return_json_data))
        .route(
            "/",
            get(|| async { "You're using axum. Try this endpoint: '/return-json-data'" }),
        )
        .route("/create-one-row", post(create_one_row))
        .route("/read-one-row", get(read_one_row))
}
