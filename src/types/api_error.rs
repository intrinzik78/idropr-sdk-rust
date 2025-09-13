use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ApiError {
    pub code: u32,
    pub message: String
}