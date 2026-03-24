use std::sync::Arc;

use reqwest::Request;

use super::config::SigOksConfig;

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
        headers.insert(
            "AccessKey",
            self.config.access_key.clone().try_into().map_err(|_| ())?,
        );
        headers.insert(
            "SecretKey",
            self.config.secret_key.clone().try_into().map_err(|_| ())?,
        );
        Ok(())
    }
}
