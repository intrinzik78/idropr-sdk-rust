use serde::{Deserialize};

use crate::types::session::ApiError;

#[derive(Debug,Deserialize)]
pub enum ApiResponse<T,U> {
    Ok(T),
    Error(U)
}

impl <T> ApiResponse<T,ApiError> {
    /// transforms reqwest status code objects to formatted ApiResponse
    pub fn parse_error(status: reqwest::StatusCode) -> ApiResponse<T,ApiError> {
        let code = status.as_u16() as u32;
        let message = status
            .canonical_reason()
            .or(Some("unknown error"))
            .unwrap()
            .to_owned();

        ApiResponse::Error(
            ApiError {code,message}
        )
    }
}