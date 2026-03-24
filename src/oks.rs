use serde::{Serializer, ser::SerializeSeq};
use tower::{Layer as _, Service as _};

pub type ApiError = crate::errors::Error<ErrorResponse>;

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        ApiError::Applicative(err)
    }
}

trait ClonableService:
    tower::Service<
        reqwest::Request,
        Response = reqwest::Response,
        Error = reqwest::Error,
        Future = futures::future::BoxFuture<'static, Result<reqwest::Response, reqwest::Error>>,
    > + Send
{
    fn cloned_box(&self) -> Box<dyn ClonableService + Send>;
}

impl<T> ClonableService for T
where
    T: tower::Service<
            reqwest::Request,
            Response = reqwest::Response,
            Error = reqwest::Error,
            Future = futures::future::BoxFuture<'static, Result<reqwest::Response, reqwest::Error>>,
        > + Clone
        + Send
        + 'static,
{
    fn cloned_box(&self) -> Box<dyn ClonableService + Send> {
        Box::new(self.clone())
    }
}

pub struct Client {
    base_url: reqwest::Url,
    inner: Box<dyn ClonableService + Send>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            base_url: self.base_url.clone(),
            inner: self.inner.cloned_box(),
        }
    }
}

impl Client {
    pub fn new(profile: &super::Profile) -> Result<Self, super::Error> {
        let client = reqwest::Client::new();
        let base_url = reqwest::Url::parse(&profile.endpoints.oks)
            .map_err(|_| super::Error::InvalidBaseUrl)?;

        if let Some((ak, sk)) = profile.access_key.as_ref().zip(profile.secret_key.as_ref()) {
            let config = crate::signoks::SigOksConfig {
                access_key: ak.to_string(),
                secret_key: sk.to_string(),
            };

            let inner = Box::new(crate::signoks::SigOksLayer::new(config.clone()).layer(client));

            Ok(Self { base_url, inner })
        } else {
            Err(super::Error::UnsupportedFeature(
                "Unknown authentication method".to_string(),
            ))
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/oks.rs"));
