use reqwest::{Client as HttpClient, RequestBuilder, Url};

use crate::{
    enums::{Auth,SDKError},
    types::{Configuration, DocExtractionClient, SecretClient, SessionsClient}
};

/// thin wrapper around a base configuration and reqwest client
#[derive(Clone,Debug)]
pub struct Client {
    cfg: Configuration,
    http: HttpClient
}

/// builder, setters, getters, accessors
impl Client {
    /// quickly configures a reqwest client with sane settings
    pub fn new(base_url: &Url, port: u16) -> Self {
        let cfg = Configuration::builder()
            .with_base_url(base_url)
            .with_port(port)
            .with_timeout_secs(30)
            .finish()
            .expect("expected ok->http client");
        
        let http = HttpClient::new();

        Self { cfg, http }
    }

    /// getter for base configuration
    pub fn cfg(&self) -> &Configuration {
        &self.cfg
    }

    /// getter for reqwest client
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// secretes accessor
    pub fn secrets(&self) -> SecretClient<'_> {
        SecretClient::new(self)
    }

    /// sessions accessor
    pub fn sessions(&self) -> SessionsClient<'_> {
        SessionsClient::new(self)
    }

    /// extractions accessor
    pub fn extractions(&self) -> DocExtractionClient<'_> {
        DocExtractionClient::new(self)
    }

    /// setter for the required access token for private and / or permission controlled endpoints
    pub fn set_access_token(&mut self, auth: Auth) {
        self.cfg.set_access_token(auth);
    }
}

/// utility functions for accessors
impl Client {
    /// utility method to set the auth header when available
    pub fn auth_req(&self, req: RequestBuilder) -> RequestBuilder {
        match self.cfg().auth() {
            Auth::None => req,
            Auth::Bearer(access_token) => req.bearer_auth(access_token),
        }
    }

    /// utility [delete] for a pre-configured reqwest HttpBuilder query
    pub fn delete(&self, endpoint: &str) -> Result<RequestBuilder,SDKError> {
        let url = self.url(endpoint)?;
        
        let req = self.http
            .delete(url)
            .timeout(self.cfg.timeout());

        Ok(req)
    }

    /// utility [get] for a pre-configured reqwest HttpBuilder query
    pub fn get(&self, endpoint: &str) -> Result<RequestBuilder,SDKError> {
        let url = self.url(endpoint)?;
        
        let req = self.http
            .get(url)
            .timeout(self.cfg.timeout());

        Ok(req)
    }

    /// utility [post] for a pre-configured reqwest HttpBuilder query
    pub fn post(&self, endpoint: &str) -> Result<RequestBuilder, SDKError> {
        let url = self.url(endpoint)?;
        let req = self.http
            .post(url)
            .timeout(self.cfg.timeout());

        Ok(req)
    }

    /// utility method to combine the accessor endpoint with the base url set on the cfg property
    pub fn url(&self, endpoint: &str) -> Result<Url,SDKError> {
        let mut url = self.cfg
            .base_url()
            .join(endpoint)
            .map_err(|_e| SDKError::UrlParsePath)?;

        let port = Some(self.cfg.port());
        
        url.set_port(port).map_err(|_e| SDKError::UrlPortParse)?;

        Ok(url)
    }
}


#[cfg(test)]
mod session_tests {
    use std::time::Duration;

    use super::*;

    #[test]
    pub fn new_client() {
        // build test components
        let url:Url = Url::parse("https://127.0.0.1").unwrap();
        let mut client = Client::new(&url, 1000);
        let duration_test = Duration::from_secs(30);

        // test base config getters
        assert_eq!(client.cfg.port(), 1000);
        assert_eq!(client.cfg.base_url(), &url);
        assert_eq!(client.cfg.timeout(),duration_test);

        // test empty auth retreival
        let auth_none_test = Auth::None;
        client.cfg.set_access_token(auth_none_test.clone());
        assert_eq!(client.cfg.auth(), &auth_none_test); 

        // test auth retreival with bearer token
        let auth_bearer_test = Auth::Bearer(String::from("123"));
        client.cfg.set_access_token(auth_bearer_test.clone());
        assert_eq!(client.cfg.auth(), &auth_bearer_test);

        // test join method to assemble the api server base url with the resource endpoint
        let endpoint_str = "/testing";
        let endpoint = client.url(endpoint_str).unwrap();
        assert_eq!(endpoint.as_str(), "https://127.0.0.1:1000/testing");
    }
}