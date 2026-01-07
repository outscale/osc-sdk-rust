use crate::apis::{
    configuration::Configuration,
    middleware::{BackoffParams, LimiterParams},
};

pub struct Endpoint {
    pub api: String,
    pub fcu: String,
    pub lbu: String,
    pub eim: String,
    pub icu: String,
    pub oos: String,
}

/// builder for constructing an endpoint configuration.
///
/// this struct is used to configure the various api endpoints,
/// allowing for overrides via environment variables or explicit setting
#[derive(Deserialize, Default)]
pub struct EndpointBuilder {
    api: Option<String>,
    fcu: Option<String>,
    lbu: Option<String>,
    eim: Option<String>,
    icu: Option<String>,
    oos: Option<String>,
}

impl EndpointBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env(self) -> Result<Self> {
        macro_rules! get_from_env {
            ($field:ident, $env:literal) => {
                match std::env::var($env) {
                    Ok(v) => Some(v),
                    Err(std::env::VarError::NotPresent) => self.$field,
                    _ => {
                        return Err(ConfigurationFileError::InvalidEnvironmentVariable(
                            $env.to_string(),
                        ))
                    }
                }
            };
        }

        Ok(Self {
            api: get_from_env!(api, "OSC_ENDPOINT_API"),
            fcu: get_from_env!(fcu, "OSC_ENDPOINT_FCU"),
            lbu: get_from_env!(lbu, "OSC_ENDPOINT_LBU"),
            eim: get_from_env!(eim, "OSC_ENDPOINT_EIM"),
            icu: get_from_env!(icu, "OSC_ENDPOINT_ICU"),
            oos: get_from_env!(oos, "OSC_ENDPOINT_OOS"),
        })
    }

    pub fn build(
        self,
        protocol: impl ToString + std::fmt::Display,
        region: impl ToString + std::fmt::Display,
    ) -> Endpoint {
        Endpoint {
            api: self
                .api
                .unwrap_or_else(|| format!("{}://api.{}.outscale.com/api/v1", protocol, region)),
            fcu: self
                .fcu
                .unwrap_or_else(|| format!("{}://fcu.{}.outscale.com", protocol, region)),
            lbu: self
                .lbu
                .unwrap_or_else(|| format!("{}://lbu.{}.outscale.com", protocol, region)),
            eim: self
                .eim
                .unwrap_or_else(|| format!("{}://eim.{}.outscale.com", protocol, region)),
            icu: self
                .icu
                .unwrap_or_else(|| format!("{}://icu.{}.outscale.com", protocol, region)),
            oos: self
                .oos
                .unwrap_or_else(|| format!("{}://oos.{}.outscale.com", protocol, region)),
        }
    }
}

pub struct Profile {
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub x509_client_cert: Option<String>,
    pub x509_client_key: Option<String>,
    pub x509_client_cert_b64: Option<String>,
    pub x509_client_key_b64: Option<String>,
    pub tls_skip_verify: bool,
    pub login: Option<String>,
    pub password: Option<String>,
    pub protocol: String,
    pub region: String,
    pub endpoints: Endpoint,
    pub backoff_params: BackoffParams,
    pub limiter_params: LimiterParams,
}

impl Profile {
    #[inline]
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::new()
    }

    #[inline]
    pub fn default() -> Result<Profile> {
        Ok(ProfileBuilder::from_standard_configuration(None, None)?.build())
    }
}

/// builder for constructing a profile.
///
/// this struct is used to configure the various parameters,
/// allowing for overrides via environment variables or explicit setting
#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ProfileBuilder {
    access_key: Option<String>,
    secret_key: Option<String>,
    x509_client_cert: Option<String>,
    x509_client_key: Option<String>,
    x509_client_cert_b64: Option<String>,
    x509_client_key_b64: Option<String>,
    tls_skip_verify: Option<bool>,
    login: Option<String>,
    password: Option<String>,
    protocol: Option<String>,
    region: Option<String>,
    endpoints: EndpointBuilder,
    backoff_params: Option<BackoffParams>,
    limiter_params: Option<LimiterParams>,
}

impl ProfileBuilder {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    pub fn from_env(self) -> Result<Self> {
        macro_rules! get_from_env {
            ($field:ident, $env:literal) => {
                match std::env::var($env) {
                    Ok(v) => Some(v),
                    Err(std::env::VarError::NotPresent) => self.$field,
                    _ => {
                        return Err(ConfigurationFileError::InvalidEnvironmentVariable(
                            $env.to_string(),
                        ))
                    }
                }
            };
        }

        Ok(Self {
            access_key: get_from_env!(access_key, "OSC_ACCESS_KEY"),
            secret_key: get_from_env!(secret_key, "OSC_SECRET_KEY"),
            x509_client_cert: get_from_env!(x509_client_cert, "OSC_X509_CLENT_CERT"),
            x509_client_key: get_from_env!(x509_client_key, "OSC_X509_CLENT_KEY"),
            x509_client_cert_b64: get_from_env!(x509_client_cert_b64, "OSC_X509_CLENT_CERT_B64"),
            x509_client_key_b64: get_from_env!(x509_client_key_b64, "OSC_X509_CLENT_KEY_B64"),
            tls_skip_verify: match std::env::var("OSC_TLS_SKIP_VERIFY") {
                Ok(e) if e.to_lowercase() == "true" => Some(true),
                Ok(_) => Some(false),
                Err(std::env::VarError::NotPresent) => self.tls_skip_verify,
                Err(_) => {
                    return Err(ConfigurationFileError::InvalidEnvironmentVariable(
                        "OSC_TLS_SKIP_VERIFY".to_string(),
                    ))
                }
            },
            login: get_from_env!(login, "OSC_LOGIN"),
            password: get_from_env!(password, "OSC_PASSWORD"),
            protocol: get_from_env!(protocol, "OSC_PROTOCOL"),
            region: get_from_env!(region, "OSC_REGION"),
            endpoints: self.endpoints.from_env()?,
            backoff_params: None,
            limiter_params: None,
        })
    }

    pub fn access_key(mut self, access_key: impl ToString, secret_key: impl ToString) -> Self {
        self.access_key = Some(access_key.to_string());
        self.secret_key = Some(secret_key.to_string());
        self
    }

    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    pub fn x509_client(mut self, client_cert: impl ToString, client_key: impl ToString) -> Self {
        self.x509_client_cert = Some(client_cert.to_string());
        self.x509_client_key = Some(client_key.to_string());
        self
    }

    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    pub fn x509_client_b64(
        mut self,
        client_cert: impl ToString,
        client_key: impl ToString,
    ) -> Self {
        self.x509_client_cert_b64 = Some(client_cert.to_string());
        self.x509_client_key_b64 = Some(client_key.to_string());
        self
    }

    pub fn basic_auth(mut self, login: impl ToString, password: impl ToString) -> Self {
        self.login = Some(login.to_string());
        self.password = Some(password.to_string());
        self
    }

    pub fn protocol(mut self, protocol: impl ToString) -> Self {
        self.protocol = Some(protocol.to_string());
        self
    }

    pub fn region(mut self, region: impl ToString) -> Self {
        self.region = Some(region.to_string());
        self
    }

    pub fn from_file(path: impl AsRef<std::path::Path>, name: impl AsRef<str>) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let mut profile_file: std::collections::HashMap<String, ProfileBuilder> =
            serde_json::from_reader(reader)?;

        match profile_file.remove(name.as_ref()) {
            Some(u) => Ok(u),
            None => Err(ConfigurationFileError::ProfileNotFound),
        }
    }

    pub fn from_standard_configuration(
        path: impl Into<Option<std::path::PathBuf>>,
        name: impl Into<Option<String>>,
    ) -> Result<Self> {
        let profile_path = {
            let mut profile_path: Option<std::path::PathBuf> = path.into();
            if profile_path.is_none() {
                profile_path = match std::env::var("OSC_CONFIG_FILE") {
                    Ok(v) => Some(std::path::PathBuf::from(v)),
                    Err(std::env::VarError::NotPresent) => None,
                    Err(_) => {
                        return Err(ConfigurationFileError::InvalidEnvironmentVariable(
                            "OSC_CONFIG_FILE".to_string(),
                        ))
                    }
                }
            }

            if profile_path.is_none() {
                if let Some(mut path) = home::home_dir() {
                    path.push(".osc/config.json");

                    if path.exists() {
                        profile_path = Some(path);
                    }
                }
            }

            profile_path
        };

        let profile_name = {
            let mut profile_name: Option<String> = name.into();
            if profile_name.is_none() {
                profile_name = match std::env::var("OSC_PROFILE") {
                    Ok(v) => Some(v),
                    Err(std::env::VarError::NotPresent) => None,
                    Err(_) => {
                        return Err(ConfigurationFileError::InvalidEnvironmentVariable(
                            "OSC_PROFILE".to_string(),
                        ))
                    }
                }
            }
            profile_name.unwrap_or_else(|| "default".to_string())
        };

        if let Some(profile_path) = profile_path {
            Self::from_file(&profile_path, &profile_name)?.from_env()
        } else {
            Self::default().from_env()
        }
    }

    pub fn build(self) -> Profile {
        let protocol = self.protocol.unwrap_or_else(|| "https".to_string());
        let region = self.region.unwrap_or_else(|| "eu-west-2".to_string());
        let endpoints = self.endpoints.build(&protocol, &region);

        Profile {
            access_key: self.access_key,
            secret_key: self.secret_key,
            x509_client_cert: self.x509_client_cert,
            x509_client_key: self.x509_client_key,
            x509_client_cert_b64: self.x509_client_cert_b64,
            x509_client_key_b64: self.x509_client_key_b64,
            tls_skip_verify: self.tls_skip_verify.unwrap_or_default(),
            login: self.login,
            password: self.password,
            protocol,
            region,
            endpoints,
            backoff_params: self.backoff_params.unwrap_or_default(),
            limiter_params: self.limiter_params.unwrap_or_default(),
        }
    }
}

impl TryFrom<Profile> for Configuration {
    type Error = ConfigurationFileError;

    fn try_from(value: Profile) -> std::result::Result<Self, Self::Error> {
        let mut client_builder =
            reqwest::blocking::Client::builder().min_tls_version(reqwest::tls::Version::TLS_1_2);

        if value.tls_skip_verify {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        {
            if let Some((x509_client_key, x509_client_cert)) =
                value.x509_client_key.zip(value.x509_client_cert)
            {
                let cert = std::fs::read(x509_client_cert)?;
                let key = std::fs::read(x509_client_key)?;
                let pkcs8 = reqwest::Identity::from_pkcs8_pem(&cert, &key)
                    .map_err(ConfigurationFileError::InvalidClientCertificate)?;
                client_builder = client_builder.identity(pkcs8);
            } else if let Some((x509_client_key_b64, x509_client_cert_b64)) =
                value.x509_client_key_b64.zip(value.x509_client_cert_b64)
            {
                use base64::engine::{general_purpose::STANDARD, Engine as _};

                let cert = STANDARD.decode(x509_client_cert_b64)?;
                let key = STANDARD.decode(x509_client_key_b64)?;
                let pkcs8 = reqwest::Identity::from_pkcs8_pem(&cert, &key)
                    .map_err(ConfigurationFileError::InvalidClientCertificate)?;
                client_builder = client_builder.identity(pkcs8);
            }
        }

        let mut config = Configuration {
            base_path: value.endpoints.api,
            client: super::middleware::ClientWithBackoff::new(
                client_builder.build().unwrap(),
                value.backoff_params.clone(),
                value.limiter_params.clone(),
            ),
            ..Default::default()
        };

        if let Some((access_key, secret_key)) = value.access_key.zip(value.secret_key) {
            config.aws_v4_key = Some(super::configuration::AWSv4Key {
                access_key,
                secret_key: secret_key.into(),
                region: value.region,
                service: "oapi".to_string(),
            })
        } else if let Some((login, password)) = value.login.zip(value.password) {
            config.basic_auth = Some((login, Some(password)));
        }

        Ok(config)
    }
}

#[derive(Debug)]
pub enum ConfigurationFileError {
    ProfileNotFound,
    Io(std::io::Error),
    Json(serde_json::Error),
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    Base64(base64::DecodeError),
    InvalidEnvironmentVariable(String),
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    InvalidClientCertificate(reqwest::Error),
}

impl std::fmt::Display for ConfigurationFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigurationFileError::ProfileNotFound => write!(f, "profile not found"),
            ConfigurationFileError::Io(e) => write!(f, "IO error: {}", e),
            ConfigurationFileError::Json(e) => write!(f, "JSON error: {}", e),
            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            ConfigurationFileError::Base64(e) => write!(f, "Base64 error: {}", e),
            ConfigurationFileError::InvalidEnvironmentVariable(v) => {
                write!(f, "invalid environment variable {}", v)
            }
            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            ConfigurationFileError::InvalidClientCertificate(e) => {
                write!(f, "invalid client certificate: {}", e)
            }
        }
    }
}

impl From<std::io::Error> for ConfigurationFileError {
    fn from(error: std::io::Error) -> Self {
        ConfigurationFileError::Io(error)
    }
}

impl From<serde_json::Error> for ConfigurationFileError {
    fn from(error: serde_json::Error) -> Self {
        match error.classify() {
            serde_json::error::Category::Io => ConfigurationFileError::Io(error.into()),
            _ => ConfigurationFileError::Json(error),
        }
    }
}

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
impl From<base64::DecodeError> for ConfigurationFileError {
    fn from(error: base64::DecodeError) -> Self {
        ConfigurationFileError::Base64(error)
    }
}

impl std::error::Error for ConfigurationFileError {}

pub type Result<T> = std::result::Result<T, ConfigurationFileError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_endpoint_builder_default() {
        let builder = EndpointBuilder::new();
        let endpoint = builder.build("https", "eu-west-2");

        assert_eq!(endpoint.api, "https://api.eu-west-2.outscale.com/api/v1");
        assert_eq!(endpoint.fcu, "https://fcu.eu-west-2.outscale.com");
        assert_eq!(endpoint.lbu, "https://lbu.eu-west-2.outscale.com");
        assert_eq!(endpoint.eim, "https://eim.eu-west-2.outscale.com");
        assert_eq!(endpoint.icu, "https://icu.eu-west-2.outscale.com");
        assert_eq!(endpoint.oos, "https://oos.eu-west-2.outscale.com");
    }

    #[test]
    fn test_endpoint_builder_from_env() {
        env::set_var("OSC_ENDPOINT_API", "https://api.custom.com");
        env::set_var("OSC_ENDPOINT_FCU", "https://fcu.custom.com");

        let builder = EndpointBuilder::new();
        let updated_builder = builder.from_env().unwrap();
        let endpoint = updated_builder.build("https", "eu-west-2");

        assert_eq!(endpoint.api, "https://api.custom.com");
        assert_eq!(endpoint.fcu, "https://fcu.custom.com");
    }

    #[test]
    fn test_profile_builder_default() {
        let builder = ProfileBuilder::new();
        let profile = builder.build();

        assert_eq!(profile.protocol, "https");
        assert_eq!(profile.region, "eu-west-2");
    }

    #[test]
    fn test_profile_builder_from_env() {
        env::set_var("OSC_ACCESS_KEY", "test_key");
        env::set_var("OSC_SECRET_KEY", "test_secret");

        let builder = ProfileBuilder::new();
        let updated_builder = builder.from_env().unwrap();

        assert_eq!(updated_builder.access_key.unwrap(), "test_key");
        assert_eq!(updated_builder.secret_key.unwrap(), "test_secret");
    }
    #[test]
    fn test_full_profile_build() {
        let profile = ProfileBuilder::new()
            .access_key("my_access_key", "my_secret_key")
            .protocol("http")
            .region("us-west-1")
            .build();

        assert_eq!(profile.access_key.unwrap(), "my_access_key");
        assert_eq!(profile.secret_key.unwrap(), "my_secret_key");
        assert_eq!(profile.protocol, "http");
        assert_eq!(profile.region, "us-west-1");
    }
}
