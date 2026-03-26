use secrecy::SecretString;
use serde::Deserialize;

pub struct Endpoint {
    pub api: String,
    pub fcu: String,
    pub lbu: String,
    pub eim: String,
    pub icu: String,
    pub oos: String,
    pub oks: String,
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
    oks: Option<String>,
}

impl EndpointBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env(self) -> Result<Self, super::Error> {
        macro_rules! get_from_env {
            ($field:ident, $env:literal) => {
                match std::env::var($env) {
                    Ok(v) => Some(v),
                    Err(std::env::VarError::NotPresent) => self.$field,
                    Err(e) => {
                        return Err(super::Error::InvalidEnvironmentVariable {
                            name: $env.to_string(),
                            reason: Some(e.to_string()),
                        })
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
            oks: get_from_env!(oks, "OSC_ENDPOINT_OKS"),
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
            oks: self.oks.unwrap_or_else(|| {
                format!("{}://api.{}.oks.outscale.com/api/v2", protocol, region)
            }),
        }
    }
}

pub struct Profile {
    pub access_key: Option<SecretString>,
    pub secret_key: Option<SecretString>,
    pub x509_client_cert: Option<String>,
    pub x509_client_key: Option<String>,
    pub x509_client_cert_b64: Option<SecretString>,
    pub x509_client_key_b64: Option<SecretString>,
    pub tls_skip_verify: bool,
    pub login: Option<SecretString>,
    pub password: Option<SecretString>,
    pub protocol: String,
    pub region: String,
    pub endpoints: Endpoint,
}

impl Profile {
    #[inline]
    pub fn builder() -> ProfileBuilder {
        ProfileBuilder::new()
    }

    #[inline]
    pub fn default() -> Result<Profile, super::Error> {
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
    default: Option<bool>,
    access_key: Option<SecretString>,
    secret_key: Option<SecretString>,
    x509_client_cert: Option<String>,
    x509_client_key: Option<String>,
    x509_client_cert_b64: Option<SecretString>,
    x509_client_key_b64: Option<SecretString>,
    tls_skip_verify: Option<bool>,
    login: Option<SecretString>,
    password: Option<SecretString>,
    protocol: Option<String>,
    region: Option<String>,
    endpoints: EndpointBuilder,
}

impl ProfileBuilder {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    pub fn from_env(self) -> Result<Self, super::Error> {
        macro_rules! get_from_env {
            ($field:ident, $env:literal) => {
                match std::env::var($env) {
                    Ok(v) => Some(v.into()),
                    Err(std::env::VarError::NotPresent) => self.$field,
                    Err(e) => {
                        return Err(super::Error::InvalidEnvironmentVariable {
                            name: $env.to_string(),
                            reason: Some(e.to_string()),
                        })
                    }
                }
            };
        }

        Ok(Self {
            default: Some(false),
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
                Err(e) => {
                    return Err(super::Error::InvalidEnvironmentVariable {
                        name: "OSC_TLS_SKIP_VERIFY".to_string(),
                        reason: Some(e.to_string()),
                    });
                }
            },
            login: get_from_env!(login, "OSC_LOGIN"),
            password: get_from_env!(password, "OSC_PASSWORD"),
            protocol: get_from_env!(protocol, "OSC_PROTOCOL"),
            region: get_from_env!(region, "OSC_REGION"),
            endpoints: self.endpoints.from_env()?,
        })
    }

    pub fn access_key(mut self, access_key: impl ToString, secret_key: impl ToString) -> Self {
        self.access_key = Some(access_key.to_string().into());
        self.secret_key = Some(secret_key.to_string().into());
        self
    }

    pub fn basic_auth(mut self, login: impl ToString, password: impl ToString) -> Self {
        self.login = Some(login.to_string().into());
        self.password = Some(password.to_string().into());
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

    pub fn from_file(
        path: impl AsRef<std::path::Path>,
        name: impl AsRef<str>,
    ) -> Result<Self, super::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let mut profile_file: std::collections::HashMap<String, ProfileBuilder> =
            serde_json::from_reader(reader)?;

        match profile_file.remove(name.as_ref()) {
            Some(u) => Ok(u),
            None => Err(super::Error::ProfileNotFound(name.as_ref().to_string())),
        }
    }

    pub fn from_standard_configuration(
        path: impl Into<Option<std::path::PathBuf>>,
        name: impl Into<Option<String>>,
    ) -> Result<Self, super::Error> {
        let profile_path = {
            let mut profile_path: Option<std::path::PathBuf> = path.into();
            if profile_path.is_none() {
                profile_path = match std::env::var("OSC_CONFIG_FILE") {
                    Ok(v) => Some(std::path::PathBuf::from(v)),
                    Err(std::env::VarError::NotPresent) => None,
                    Err(e) => {
                        return Err(super::Error::InvalidEnvironmentVariable {
                            name: "OSC_CONFIG_FILE".to_string(),
                            reason: Some(e.to_string()),
                        });
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
                    Err(e) => {
                        return Err(super::Error::InvalidEnvironmentVariable {
                            name: "OSC_PROFILE".to_string(),
                            reason: Some(e.to_string()),
                        });
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
        }
    }
}
