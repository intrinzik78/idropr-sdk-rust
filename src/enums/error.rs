use derive_more::derive::From;
use std::{
    fmt::Display,
    string::FromUtf8Error
};
use reqwest;

#[derive(Debug,From)]
pub enum SDKError {
    #[from]
    Http(reqwest::Error),

    #[from]
    Base64(base64::DecodeError),
    
    /// Utf8 errors are generated during decryption when Vec<u8> is converted to plain text
    #[from]
    FromUtf8Error(FromUtf8Error),

    // SDK defined errors ↴
    FailedDeserialization,    // generated when an OK response is received, but parsing failed
    ServerConnectionFailed,   // generated when a query execution is attempted, but a connection fails
    RequestRejected,
    UrlParsePath,
    UrlPortParse,
    
    
    // disabled by default ↴
    DevError(String),
}

impl std::error::Error for SDKError {}

impl Display for SDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        type S = SDKError;
        // print only on non-production server modes, otherwise do not print detailed
        match self {
            S::FailedDeserialization => write!(f,"OK response is received, but failed to parse response."),
            S::ServerConnectionFailed => write!(f,"could not connect to server"),
            S::RequestRejected => write!(f,"server rejected request"),
            S::UrlParsePath => write!(f,"malformed path, parsing failed"),
            S::UrlPortParse => write!(f,"malformed port, parsing failed"),
            _ => write!(f, "{self:?}")
        }
    }
}
