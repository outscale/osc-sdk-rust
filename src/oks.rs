use secrecy::ExposeSecret as _;
use serde::{Serializer, ser::SerializeSeq};
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
        let client = {
            let mut client_builder = crate::policy::default_pooled_transport();

            if profile.tls_skip_verify {
                client_builder = client_builder.danger_accept_invalid_certs(true);
            }

            #[cfg(feature = "default-tls")]
            fn mk_identity(
                mut key: Vec<u8>,
                mut cert: Vec<u8>,
            ) -> Result<reqwest::tls::Identity, super::Error> {
                key.append(&mut cert);
                reqwest::Identity::from_pem(&key)
                    .map_err(|e| crate::Error::InvalidClientCertificate(e.to_string()))
            }

            #[cfg(feature = "native-tls")]
            fn mk_identity(
                key: Vec<u8>,
                cert: Vec<u8>,
            ) -> Result<reqwest::tls::Identity, super::Error> {
                key.append(cert);
                reqwest::Identity::from_pkcs8_pem(&key, &cert)
                    .map_err(crate::Error::InvalidClientCertificate)
            }

            #[cfg(not(any(feature = "default-tls", feature = "native-tls")))]
            fn mk_identity(
                _: Vec<u8>,
                cert: Vec<u8>,
            ) -> Result<reqwest::tls::Identity, super::Error> {
                Err(super::Error::UnsupportedFeature(
                    "mTLS required rustls-tls or native-tls feature flag".to_string(),
                ))
            }

            if let Some((x509_client_key, x509_client_cert)) = profile
                .x509_client_key
                .as_ref()
                .zip(profile.x509_client_cert.as_ref())
            {
                let cert = std::fs::read(x509_client_cert)?;
                let key = std::fs::read(x509_client_key)?;
                let pkcs8 = mk_identity(key, cert)?;
                client_builder = client_builder.identity(pkcs8);
            } else if let Some((x509_client_key_b64, x509_client_cert_b64)) = profile
                .x509_client_key_b64
                .as_ref()
                .zip(profile.x509_client_cert_b64.as_ref())
            {
                use base64::engine::{Engine as _, general_purpose::STANDARD};

                let cert = STANDARD.decode(x509_client_cert_b64.expose_secret())?;
                let key = STANDARD.decode(x509_client_key_b64.expose_secret())?;
                let pkcs8 = mk_identity(key, cert)?;
                client_builder = client_builder.identity(pkcs8);
            }

            client_builder.build()
        }?;

        let base_url = reqwest::Url::parse(&profile.endpoints.oks)
            .map_err(|_| super::Error::InvalidBaseUrl)?;

        if let Some((ak, sk)) = profile.access_key.as_ref().zip(profile.secret_key.as_ref()) {
            let config = crate::signv4::SigV4Config {
                access_key: ak.clone(),
                secret_key: sk.clone(),
                region: profile.region.clone(),
                service: "oks".to_string(),
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
        } else if let Some((login, password)) =
            profile.login.as_ref().zip(profile.password.as_ref())
        {
            let config = crate::basicauth::BasicAuthConfig {
                username: login.clone(),
                password: password.clone(),
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
                .layer(crate::basicauth::BasicAuthLayer::new(config.clone()))
                .service(client);

            Ok(Self { base_url, inner })
        } else {
            Err(super::Error::UnsupportedFeature(
                "Unknown authentication method".to_string(),
            ))
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/oks.rs"));
