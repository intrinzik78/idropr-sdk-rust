use std::time::Duration;
use reqwest::Url;

use crate::enums::Auth;

/// used by the Client struct as a configuration for HTTP calls to the API
#[derive(Clone,Debug)]
pub struct Configuration {
    auth: Auth,
    base_url: Url,
    port: u16,
    timeout: Duration
}

#[derive(Clone,Debug)]
pub struct Builder {
    pub auth: Auth,
    pub base_url: Option<Url>,
    pub port: Option<u16>,
    pub timeout_secs: Option<u64>
}

impl Configuration {
    /// setter for the required access token for private and / or permission controlled endpoints
    pub fn set_access_token(&mut self, auth: Auth) {
        self.auth = auth;
    }

    /// getter, reference to the access token
    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    /// getter for api base url
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// getter for api sever port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// getter for timeout param
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// empty configuration container, use builder functions to set params
    pub fn builder() -> Builder {
        Builder {
            auth: Auth::None,
            base_url: None,
            port: None,
            timeout_secs: None
        }
    }
}

impl Builder {
    /// sets the early shortcircuit timeout param
    pub fn with_auth(mut self, auth: Auth) -> Self {
        self.auth = auth;

        self
    }

    /// sets the target server url base, final endpoint url is determined by the target function
    pub fn with_base_url(mut self, base_url: &Url) -> Self {
        self.base_url = Some(base_url.to_owned());

        self
    }

    /// sets the target server port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);

        self
    }

    /// sets the early shortcircuit timeout param
    pub fn with_timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);

        self
    }

    pub fn finish(&self) -> Result<Configuration, &'static str> {
        let base_url = self.base_url
            .clone()
            .ok_or("cannont configure the client, missing base server url")?;
        let port = self.port
            .ok_or("cannot configure the client, missing server port")?;
        let timeout_secs = self.timeout_secs
            .unwrap_or(30);
        let duration = Duration::from_secs(timeout_secs);

        Ok(Configuration {
            auth: self.auth.clone(),
            base_url,
            port,
            timeout: duration
        })
    }
}