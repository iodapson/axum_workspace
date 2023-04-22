use dotenvy::dotenv;
//use std::error::Error;

pub struct SomeTableA {
    pub column_1: String,
    pub column_2: String,
}

pub async fn create_one_row() {
    dotenv().ok();

    // extract database url into a variable
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let postgres_instance_connection_pool = sqlx::postgres::PgPool::connect(&database_url.as_str())
        .await
        .unwrap();
    // Create a new SQL query to create a table
    let sql_query = "
        INSERT INTO some_table (column_1, column_2)
        VALUES ('draft', '$51')
    ";

    let some_table_a_row = SomeTableA {
        column_1: "Check".to_string(),
        column_2: "$100".to_string(),
    };

    // Execute the above sql query named 'sql_query'
    let _successful_query = sqlx::query(sql_query)
        .bind(&some_table_a_row.column_1)
        .bind(&some_table_a_row.column_2)
        .execute(&postgres_instance_connection_pool)
        .await
        .unwrap();
}
