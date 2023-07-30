use axum::response::Html;

pub async fn get_root_path() -> Html<String> {
    Html("
    <div style='width: auto; height: auto; background: #eee; padding: 2rem; text-align: center; border: 5px solid white;'>
        <div style='flex: 1 100%;'>
            <p>Welcome! This Rust Axum app is now Running at Port 8090</p>
        </div>
    <div>".to_owned(),
    )
}
