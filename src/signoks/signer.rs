use std::sync::Arc;

use reqwest::{Request, header::HeaderValue};

use super::config::SigOksConfig;
use secrecy::ExposeSecret;

#[derive(Clone)]
pub(crate) struct SigOksSigner {
    config: Arc<SigOksConfig>,
}

impl SigOksSigner {
    pub fn new(config: SigOksConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    pub(crate) fn sign(&self, request: &mut Request) -> Result<(), ()> {
        let headers = request.headers_mut();

        {
            let mut header = HeaderValue::from_str(self.config.access_key.expose_secret()).unwrap();
            header.set_sensitive(true);
            headers.insert("AccessKey", header);
        };

        {
            let mut header = HeaderValue::from_str(self.config.secret_key.expose_secret()).unwrap();
            header.set_sensitive(true);
            headers.insert("SecretKey", header);
        };

        Ok(())
    }
}
