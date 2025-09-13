use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct ApiSuccess<T> {
    pub code: u32,
    pub message: String,
    pub data: T
}