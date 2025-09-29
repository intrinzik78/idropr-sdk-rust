use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    enums::{ApiResponse,SDKError},
    types::{ApiSuccess,ApiError,Client}
};

type CreateResponse = ApiResponse<ApiSuccess<Option<()>>,ApiError>;
type Result<T> = std::result::Result<T,SDKError>;

#[derive(Serialize)]
pub struct CreateSecretBody {
    name: String,
    description: String,
    api_key: Option<String>,
    api_secret: Option<String>
}


pub struct SecretClient<'a> {
    client: &'a Client
}

/// canonical
impl<'a> SecretClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create_secret(&self, name: &str, description: &str, api_key_opt: Option<&String>, api_secret_opt: Option<&String>) -> Result<CreateResponse> {
        // extract and format api key
        let api_key = match api_key_opt {
            Some(s) => Some(s.to_owned()),
            None => None
        };

        // extract and format api secret
        let api_secret = match api_secret_opt {
            Some(s) => Some(s.to_owned()),
            None => None
        };

        // build query body
        let body = CreateSecretBody {
            name: name.to_owned(),
            description: description.to_owned(),
            api_key,
            api_secret
        };

        // build base query
        let req = self.client
            .post("/v1/secrets")?
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
        let response:CreateResponse = match res.status() {
            StatusCode::OK => res.json().await?,
            StatusCode::BAD_REQUEST => {
                let error: CreateResponse = res.json().await?;
                return Ok(error);
            },
            _ => {
                let error = ApiResponse::parse_error(res.status());
                return Ok(error);
            }
        };

        Ok(response)
    }
}

/// ergonomic
impl<'a> SecretClient<'a> {

}