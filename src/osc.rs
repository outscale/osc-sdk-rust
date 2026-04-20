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
    inner: super::transport::TransportService,
}

impl Client {
    pub fn new(profile: &super::Profile) -> Result<Self, super::Error> {
        let base_url = reqwest::Url::parse(&profile.endpoints.api)
            .map_err(|_| super::Error::InvalidBaseUrl)?;

        let inner = super::transport::build_service(profile, "osc")?;

        Ok(Self { base_url, inner })
    }
}

include!(concat!(env!("OUT_DIR"), "/osc.rs"));
