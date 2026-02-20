use serde::{Serializer, ser::SerializeSeq};
use tower::{Layer as _, Service as _};

pub type ApiError = crate::errors::Error<ErrorResponse>;

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        ApiError::Applicative(err)
    }
}

pub struct Client {
    base_url: reqwest::Url,
    inner: crate::signv4::service::SigV4Service<reqwest::Client>,
}

impl Client {
    pub fn new(config: &crate::signv4::SigV4Config) -> Result<Self, ()> {
        let client = reqwest::Client::new();
        let inner = crate::signv4::SigV4Layer::new(config.clone()).layer(client);

        Ok(Self {
            base_url: reqwest::Url::parse("https://api.eu-west-2.oks.outscale.com/api/v2")
                .map_err(|_| ())?,
            inner,
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/oks.rs"));
