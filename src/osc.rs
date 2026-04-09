use tower::Service as _;
use tower::ServiceExt as _;

pub type ApiError = crate::errors::Error<ErrorResponse>;

impl From<ErrorResponse> for ApiError {
    fn from(err: ErrorResponse) -> Self {
        ApiError::Applicative(err)
    }
}

impl From<tower::BoxError> for ApiError {
    fn from(_value: tower::BoxError) -> Self {
        ApiError::InvalidBaseUrl
    }
}

#[derive(Clone)]
pub struct Client {
    base_url: reqwest::Url,
    inner: tower::util::BoxCloneService<reqwest::Request, reqwest::Response, tower::BoxError>,
}

impl Client {
    pub fn new(profile: &super::Profile) -> Result<Self, super::Error> {
        let client = crate::policy::default_pooled_transport().build()?;
        let base_url = reqwest::Url::parse(&profile.endpoints.api)
            .map_err(|_| super::Error::InvalidBaseUrl)?;

        if let Some((ak, sk)) = profile.access_key.as_ref().zip(profile.secret_key.as_ref()) {
            let config = crate::signv4::SigV4Config {
                access_key: ak.clone(),
                secret_key: sk.clone(),
                region: profile.region.clone(),
                service: "osc".to_string(),
                session_token: None,
            };

            let inner = tower::ServiceBuilder::new()
                .boxed_clone()
                .buffer(1024)
                .rate_limit(5, std::time::Duration::from_secs(1))
                .retry(crate::policy::BasePolicy::new(
                    3,
                    std::time::Duration::from_millis(100),
                    std::time::Duration::from_secs(30),
                    3.0,
                ))
                .layer(crate::signv4::SigV4Layer::new(config.clone()))
                .service(client);

            Ok(Self { base_url, inner })
        } else {
            Err(super::Error::UnsupportedFeature(
                "Unknown authentication method".to_string(),
            ))
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/osc.rs"));
