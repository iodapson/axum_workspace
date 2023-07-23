pub struct SomeTableA {
    pub column_1: String,
    pub column_2: String,
}

pub async fn create_one_row() {
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
