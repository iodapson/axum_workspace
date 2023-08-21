mod root_path;

use crate::config::Config;

use axum::{routing::get, Router};
use root_path::get_root_path;
use sqlx::PgPool;
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
        .with_state(api_context)
}
