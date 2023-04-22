use axum::Extension;

#[derive(Clone)]
pub struct SharedData {
    pub data_one: String,
}

// * axum::Extension is used to wrap the target data to be shared - SharedData
pub async fn access_shared_data(Extension(extracted_shared_data): Extension<SharedData>) -> String {
    extracted_shared_data.data_one
}
