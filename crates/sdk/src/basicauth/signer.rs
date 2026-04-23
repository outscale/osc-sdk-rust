use std::sync::Arc;

use base64::Engine as _;
use log::debug;
use reqwest::{Request, header::HeaderValue};
use secrecy::ExposeSecret;

use super::config::BasicAuthConfig;

#[derive(Clone)]
pub(crate) struct BasicAuthSigner {
    config: Arc<BasicAuthConfig>,
}

impl BasicAuthSigner {
    pub fn new(config: BasicAuthConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub(crate) fn sign(&self, request: &mut Request) -> Result<(), ()> {
        let credentials = format!(
            "{}:{}",
            self.config.username.expose_secret(),
            self.config.password.expose_secret()
        );

        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials.as_bytes());
        let auth_value = format!("Basic {}", encoded);

        let mut auth_header = HeaderValue::from_str(&auth_value).map_err(|_| ())?;
        auth_header.set_sensitive(true);

        request.headers_mut().insert("Authorization", auth_header);

        debug!("Basic auth added to request");

        Ok(())
    }
}
