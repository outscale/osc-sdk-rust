use tower::{Layer as _, Service as _};

pub type ApiError = crate::errors::Error<ErrorResponse>;

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        ApiError::Applicative(err)
    }
}

pub struct Client {
    base_url: reqwest::Url,
    inner: Box<
        dyn tower::Service<
                reqwest::Request,
                Response = reqwest::Response,
                Error = reqwest::Error,
                Future = futures::future::BoxFuture<
                    'static,
                    Result<reqwest::Response, reqwest::Error>,
                >,
            >,
    >,
}

impl Client {
    pub fn new(profile: &super::Profile) -> Result<Self, super::Error> {
        let client = reqwest::Client::new();
        let base_url = reqwest::Url::parse(&profile.endpoints.api)
            .map_err(|_| super::Error::InvalidBaseUrl)?;

        if let Some((ak, sk)) = profile.access_key.as_ref().zip(profile.secret_key.as_ref()) {
            let config = crate::signv4::SigV4Config {
                access_key: ak.to_string(),
                secret_key: sk.to_string(),
                region: profile.region.clone(),
                service: "osc".to_string(),
                session_token: None,
            };

            let inner = Box::new(crate::signv4::SigV4Layer::new(config.clone()).layer(client));

            Ok(Self { base_url, inner })
        } else {
            Err(super::Error::UnsupportedFeature(
                "Unknown authentication method".to_string(),
            ))
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/osc.rs"));
