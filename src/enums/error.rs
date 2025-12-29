use derive_more::derive::From;
use reqwest::{self, StatusCode};
use std::{
    fmt::{Debug, Display},
    string::FromUtf8Error
};

use crate::types::ApiError;

#[derive(Debug,From)]
pub enum SDKError {
    #[from]
    Http(reqwest::Error),

    #[from]
    Io(std::io::Error),

    #[from]
    Base64(base64::DecodeError),
    
    /// Utf8 errors are generated during decryption when Vec<u8> is converted to plain text
    #[from]
    FromUtf8Error(FromUtf8Error),

    // SDK defined errors ↴
    ApiError(ApiError),
    FailedDeserialization,    // generated when an OK response is received, but parsing failed
    FileSizeMismatch { expected:u64, actual: u64 },
    NoContentToDeserialize,   // generated when trying to deserialize a NO_CONTENT status type
    ProcessingNoContent,      // generated when trying to deserialize a PROCESSING status type
    ResetContent,             // generated when trying to deserialize a RESET_CONTENT status type
    ServerConnectionFailed,   // generated when a query execution is attempted, but a connection fails
    RequestRejected,
    UnexpectedResponseType(StatusCode),
    UrlParsePath,
    UrlPortParse,             // 
    
    
    // disabled by default ↴
    DevError(String),
}

impl SDKError {
    pub fn from_reqwest(e: reqwest::Error) -> SDKError {
        if e.is_connect() || e.is_timeout() {
            SDKError::ServerConnectionFailed
        } else {
            SDKError::Http(e)
        }
    }
}

impl std::error::Error for SDKError {}

impl Display for SDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type S = SDKError;
        // print only on non-production server modes, otherwise do not print detailed
        match self {
            S::FailedDeserialization    => write!(f,"OK response is received, but failed to parse response."),
            S::NoContentToDeserialize   => write!(f,"Empty response object (204), no content to deserialize"),
            S::ProcessingNoContent      => write!(f,"Empty response object (102), no content to deserialize"),
            S::ResetContent             => write!(f,"Empty response object (205), no content to deserialize"),
            S::ServerConnectionFailed   => write!(f,"could not connect to server"),
            S::RequestRejected          => write!(f,"server rejected request"),
            S::UrlParsePath             => write!(f,"malformed path, parsing failed"),
            S::UrlPortParse             => write!(f,"malformed port, parsing failed"),
            _ => write!(f, "{self:?}")
        }
    }
}
