// use sqlx::Row;
use std::error::Error;
/*
pub struct SomeTableA {
    column_1: String,
    column_2: String,
} */

pub async fn run_database(database_url: String) -> Result<(), Box<dyn Error>> {
    let postgres_instance_connection_pool =
        sqlx::postgres::PgPool::connect(&database_url.as_str()).await?;

    // initialize migration of tables
    sqlx::migrate!("./migrations")
        .run(&postgres_instance_connection_pool)
        .await?;

    /*
    // instantiate a 'SomeTableA' to pass in to function 'create_a_table'
    let some_table_a = SomeTableA {
        column_1: "Check".to_string(),
        column_2: "$100".to_string(),
    };

    create_some_table_a(&some_table_a, &postgres_instance_connection_pool).await?;

    let read_some_table_a_row = read_some_table_a_row(&postgres_instance_connection_pool).await?;

    println!(
        "read_some_table_a_row.column_1 is: {}",
        read_some_table_a_row.column_1
    );
    println!(
        "read_some_table_a_row.column_2 is: {}",
        read_some_table_a_row.column_2
    ); */

    Ok(())
}
/*
async fn create_some_table_a(
    some_table_row: &SomeTableA,
    postgres_instance_pool_connection: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    // Create a new SQL query to create a table
    let sql_query = "
        INSERT INTO some_table (column_1, column_2)
        VALUES (draft, $50)
    ";

    // Execute the above sql query named 'sql_query'
    sqlx::query(sql_query)
        .bind(&some_table_row.column_1)
        .bind(&some_table_row.column_2)
        .execute(postgres_instance_pool_connection)
        .await?;

    Ok(())
}

async fn read_some_table_a_row(
    postgres_instance_connection_pool: &sqlx::PgPool,
) -> Result<SomeTableA, Box<dyn Error>> {
    let sqlx_query = "
        SELECT column_1, column_2
        FROM some_table
    ";

    // Now execute and store the read query using SQLx and without binds
    let query_result = sqlx::query(sqlx_query);

    // Choose one row from 'query_result'
    let one_some_table_row = query_result
        .fetch_one(postgres_instance_connection_pool)
        .await?;

    // Prepare a SomeTable instance to return
    let some_table_row_instance = SomeTableA {
        column_1: one_some_table_row.get("column_1"),
        column_2: one_some_table_row.get("column_2"),
    };

    Ok(some_table_row_instance)
}
*/
