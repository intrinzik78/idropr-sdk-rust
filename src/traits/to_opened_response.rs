use reqwest::StatusCode;
use serde::{de::DeserializeOwned};

use crate::{enums::{ApiResponse, SDKError}, types::ApiSuccess};

pub trait ToOpenedResponse<T> {
    fn open(self) -> impl std::future::Future<Output = Result<ApiSuccess<T>,SDKError>> + Send;
}

impl <T> ToOpenedResponse<T> for reqwest::Response
where
    T: DeserializeOwned
{
    async fn open(self) -> Result<ApiSuccess<T>,SDKError> {
        match self.status() {
            StatusCode::NO_CONTENT      => return Err(SDKError::NoContentToDeserialize),
            StatusCode::PROCESSING      => return Err(SDKError::ProcessingNoContent),
            StatusCode::RESET_CONTENT   => return Err(SDKError::ResetContent),
            _                           => {}
        }

        let response = self
            .json()
            .await
            .map_err(|_| SDKError::FailedDeserialization)?;

        match response {
            ApiResponse::Ok(s) => Ok(s),
            ApiResponse::Error(e) => Err(SDKError::ApiError(e))
        }
    }
}