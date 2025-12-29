use reqwest::StatusCode;

use crate::types::ApiError;

pub trait ToParsedError {
    fn parse_error(self) -> ApiError;
}

impl ToParsedError for StatusCode {
    fn parse_error(self) -> ApiError {
        let code = self.as_u16() as u32;
        let message = self
            .canonical_reason()
            .or(Some("unknown error"))
            .unwrap()
            .to_owned();

        let api_error = ApiError { code,message };

        api_error
    }
}

