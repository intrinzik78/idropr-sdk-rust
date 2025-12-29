use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub enum ApiResponse<T,ApiError> {
    Ok(T),
    Error(ApiError)
}