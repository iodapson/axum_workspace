use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct TargetData {
    field_one: String,
    field_two: i32,
    field_three: String,
}

pub async fn return_json_data() -> Json<TargetData> {
    let target_data = TargetData {
        field_one: "axum powered field_one".to_owned(),
        field_two: 1 as i32,
        field_three: "axum powered field_two".to_owned(),
    };
    Json(target_data)
}
