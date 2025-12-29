use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct AccessToken {
    pub access_token: String
}