mod add_new_expense;
mod root_path;

use crate::config::Config;

use add_new_expense::add_new_expense;
use axum::{
    routing::{get, post},
    Router,
};
use root_path::get_root_path;
use sqlx::PgPool; // Newly added!
                  //use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[allow(unused)]
#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub fn build_routes(config: Config, db: PgPool) -> Router {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    api_router(api_context)
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .route("/", get(get_root_path))
        .route("/create-new-expense", post(add_new_expense))
        .with_state(api_context)
}
