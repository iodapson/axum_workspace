use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn run_database(database_url: &str) -> Result<Pool<Postgres>> {
    // create a PgPoolOptions connection to DATABASE_URL
    let db_pool_options = PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
        .with_context(|| "Connection to Database failed")?;

    // Correctly embed into this application database migrations
    // ...performed by command `$ sqlx db setup`
    // ...You can use this command each time you add to dir migrations
    sqlx::migrate!()
        .run(&db_pool_options)
        .await
        .with_context(|| "SQLx DB migration failed")?;

    Ok(db_pool_options)
}
