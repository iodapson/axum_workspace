use axum::response::Html;

pub async fn add_new_expense() -> Html<String> {
    Html("<p>Congratulations. New expense added!</p>".to_owned())
}
