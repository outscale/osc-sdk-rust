use std::time::Duration;

use secrecy::ExposeSecret as _;

use crate::Profile;

static SDK_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn default_pooled_transport() -> reqwest::ClientBuilder {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .user_agent(SDK_USER_AGENT)
        .tcp_keepalive(Duration::from_secs(30))
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(90))
        .http2_adaptive_window(true)
}

fn build_transport(profile: &Profile) -> Result<reqwest::Client, super::Error> {
    let mut client_builder = default_pooled_transport();

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
        mut key: Vec<u8>,
        mut cert: Vec<u8>,
    ) -> Result<reqwest::tls::Identity, super::Error> {
        key.append(&mut cert);
        reqwest::Identity::from_pkcs8_pem(&key, &cert)
            .map_err(|e| crate::Error::InvalidClientCertificate(e.to_string()))
    }

    #[cfg(not(any(feature = "default-tls", feature = "native-tls")))]
    fn mk_identity(_: Vec<u8>, _: Vec<u8>) -> Result<reqwest::tls::Identity, super::Error> {
        Err(super::Error::UnsupportedFeature(
            "mTLS required rustls-tls or native-tls feature flag".to_string(),
        ))
    }

    // mTLS certificate handling
    // NOTE: Private key material is loaded into Vec<u8> and passed to reqwest.
    // Ideally, we would use secrecy::Secret<Vec<u8>> with zeroization, but
    // reqwest's Identity API currently requires owned Vec<u8>. The key material
    // is short-lived and passed directly to the TLS stack, minimizing exposure.
    // Future improvement: Investigate zeroizing allocator or reqwest API changes.
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

    Ok(client_builder.build()?)
}

pub(crate) type TransportService =
    tower::util::BoxCloneService<reqwest::Request, reqwest::Response, tower::BoxError>;

/// Default service configuration constants
/// These values control the behavior of the tower middleware stack
const DEFAULT_BUFFER_SIZE: usize = 1024;
const DEFAULT_RATE_LIMIT_MAX: u64 = 5;
const DEFAULT_RATE_LIMIT_PERIOD_SECS: u64 = 1;
const DEFAULT_RETRY_MAX_ATTEMPTS: usize = 3;
const DEFAULT_RETRY_INITIAL_BACKOFF_MS: u64 = 100;
const DEFAULT_RETRY_MAX_BACKOFF_SECS: u64 = 30;
const DEFAULT_RETRY_BACKOFF_MULTIPLIER: f64 = 3.0;

pub(crate) fn build_service(
    profile: &Profile,
    service: &str,
) -> Result<TransportService, super::Error> {
    let client = build_transport(profile)?;

    if let Some((ak, sk)) = profile.access_key.as_ref().zip(profile.secret_key.as_ref()) {
        let config = crate::signv4::SigV4Config {
            access_key: ak.clone(),
            secret_key: sk.clone(),
            region: profile.region.clone(),
            service: service.to_string(),
            session_token: None,
        };

        Ok(tower::ServiceBuilder::new()
            .boxed_clone()
            .buffer(DEFAULT_BUFFER_SIZE)
            .rate_limit(
                DEFAULT_RATE_LIMIT_MAX,
                std::time::Duration::from_secs(DEFAULT_RATE_LIMIT_PERIOD_SECS),
            )
            .retry(crate::policy::BasePolicy::new(
                DEFAULT_RETRY_MAX_ATTEMPTS,
                std::time::Duration::from_millis(DEFAULT_RETRY_INITIAL_BACKOFF_MS),
                std::time::Duration::from_secs(DEFAULT_RETRY_MAX_BACKOFF_SECS),
                DEFAULT_RETRY_BACKOFF_MULTIPLIER,
            )?)
            .layer(crate::signv4::SigV4Layer::new(config.clone()))
            .service(client))
    } else if let Some((login, password)) = profile.login.as_ref().zip(profile.password.as_ref()) {
        let config = crate::basicauth::BasicAuthConfig {
            username: login.clone(),
            password: password.clone(),
        };

        Ok(tower::ServiceBuilder::new()
            .boxed_clone()
            .buffer(DEFAULT_BUFFER_SIZE)
            .rate_limit(
                DEFAULT_RATE_LIMIT_MAX,
                std::time::Duration::from_secs(DEFAULT_RATE_LIMIT_PERIOD_SECS),
            )
            .retry(crate::policy::BasePolicy::new(
                DEFAULT_RETRY_MAX_ATTEMPTS,
                std::time::Duration::from_millis(DEFAULT_RETRY_INITIAL_BACKOFF_MS),
                std::time::Duration::from_secs(DEFAULT_RETRY_MAX_BACKOFF_SECS),
                DEFAULT_RETRY_BACKOFF_MULTIPLIER,
            )?)
            .layer(crate::basicauth::BasicAuthLayer::new(config.clone()))
            .service(client))
    } else {
        Err(super::Error::UnsupportedFeature(
            "Unknown authentication method".to_string(),
        ))
    }
}
