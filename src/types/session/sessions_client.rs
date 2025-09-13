use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    enums::{ApiResponse,SDKError},
    types::{AccessToken,ApiError,ApiSuccess,Client},
};

type Result<T> = std::result::Result<T,SDKError>;
type CreateResponse = ApiResponse<ApiSuccess<AccessToken>,ApiError>;
type DeleteResponse = ApiResponse<ApiSuccess<Option<()>>,ApiError>;

#[derive(Debug,Serialize)]
struct CreateSessionBody {
    username: String,
    password: String
}

#[derive(Debug)]
pub struct SessionsClient<'a> {
    client: &'a Client
}

/// canonical
impl <'a> SessionsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// canonical method to post session to api server
    pub async fn create_session(&self, username: &str, password: &str) -> Result<CreateResponse> {
        // build query body
        let body = CreateSessionBody {
            username: username.to_owned(),
            password: password.to_owned()
        };

        // build base query
        let req = self.client
            .post("/v1/sessions")?
            .json(&body);

        // append auth headers when available
        let req = self.client
            .auth_req(req)
            .build()?;

        // execute the query and format the response
        let res = self.client
            .http()
            .execute(req)
            .await
            .map_err(|_| SDKError::ServerConnectionFailed)?;

        // handle all response possibilities
        let response:CreateResponse = {
            match res.status() {
                StatusCode::OK => res.json().await.map_err(|_| SDKError::FailedDeserialization)?,
                _ => ApiResponse::parse_error(res.status())
            }
        };

        Ok(response)
    }
    
    /// canonical method to delete session from api server
    pub async fn delete_session(&self) -> Result<DeleteResponse> {
        // build base query
        let req = self.client.delete("/v1/sessions")?;

        // append auth headers when available
        let req = self.client
            .auth_req(req)
            .build()?;

        // execute the query and format the response
        let res = self.client
            .http()
            .execute(req)
            .await
            .map_err(|_| SDKError::ServerConnectionFailed)?;

        // handle all response possibilities
        let response:DeleteResponse = {
            match res.status() {
                StatusCode::OK => res.json().await.map_err(|_| SDKError::FailedDeserialization)?,
                _ => ApiResponse::parse_error(res.status())
            }
        };

        Ok(response)
    }
}

/// ergonomic
impl <'a> SessionsClient<'a> {
    /// ergonomically friendly method to log a user in to the system
    pub async fn login(&self, username: &str, password: &str) -> Result<CreateResponse> {
        self.create_session(username, password).await
    }

    /// ergonomicaly friendly method to log a user out of the system
    pub async fn logout(&self) -> Result<DeleteResponse> {
        self.delete_session().await
    }
}

#[cfg(test)]
mod session_tests {
    use reqwest::Url;

    use super::*;

    #[test]
    pub fn build_session_client() {
        let url:Url = Url::parse("https://127.0.0.1").unwrap();
        let client = Client::new(&url, 1000);
        let _sessions = SessionsClient::new(&client);
    }
}