//use axum::Extension;
use dotenvy::dotenv;
use sqlx::Row;

pub struct SomeTableA {
    pub column_1: String,
    pub column_2: String,
}

pub async fn read_one_row() -> String {
    dotenv().ok();

    // extract database url into a variable
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let postgres_instance_connection_pool = sqlx::postgres::PgPool::connect(&database_url.as_str())
        .await
        .unwrap();

    let sqlx_query = "
        SELECT column_1, column_2
        FROM some_table
    ";

    // Now execute and store the read query using SQLx and without binds
    let query_result = sqlx::query(sqlx_query);

    dbg!("reading one row...");

    // Choose one row from 'query_result'
    let one_some_table_row = query_result
        .fetch_one(&postgres_instance_connection_pool)
        .await
        .unwrap();

    // Prepare a SomeTable instance to return
    let some_table_row_instance = SomeTableA {
        column_1: one_some_table_row.get("column_1"),
        column_2: one_some_table_row.get("column_2"),
    };

    String::from(some_table_row_instance.column_1)
}
