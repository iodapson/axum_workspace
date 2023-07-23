mod add_new_expense;
mod root_path_get;

use crate::config::Config;
//use crate::run_database;
use add_new_expense::add_new_expense;
use axum::{
    routing::{get, post},
    Router,
};
use root_path_get::get_root_path;
use sqlx::PgPool; // Newly added!
                  //use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[allow(unused)]
/*pub(crate) struct ApiContext {
    pub db_pool_options_arc: Arc<Pool<Postgres>>, // remove this, and replace with a string value extracted from .env
    pub hmac_key: String,                         // hmac_key unused!
}*/
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

    let app = api_router(api_context);

    app
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        //.route("/", run_database)
        //.with_state()
        .route("/", get(get_root_path))
        .route("/create-new-expense", post(add_new_expense))
        .with_state(api_context)
}
