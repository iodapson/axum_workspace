use axum::response::Html;

pub async fn get_root_path() -> Html<String> {
    Html("<p>Welcome to the Dsquare Prototype</p>".to_owned())
}
