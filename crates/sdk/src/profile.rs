use secrecy::SecretString;
use serde::{Deserialize, de::DeserializeSeed};

#[derive(Debug)]
pub struct Endpoint {
    pub api: String,
    pub fcu: String,
    pub lbu: String,
    pub eim: String,
    pub icu: String,
    pub oos: String,
    pub oks: String,
}

fn default_aws_endpoint(protocol: &str, region: &str, service: &str) -> String {
    format!("{}://{}.{}.outscale.com", protocol, service, region)
}

fn default_oapi_endpoint(protocol: &str, region: &str) -> String {
    format!("{}://api.{}.outscale.com/api/v1", protocol, region)
}

fn default_oks_endpoint(protocol: &str, region: &str) -> String {
    format!("{}://api.{}.oks.outscale.com/api/v2", protocol, region)
}

impl Endpoint {
    fn new(protocol: &str, region: &str) -> Self {
        Self {
            api: default_oapi_endpoint(protocol, region),
            fcu: default_aws_endpoint(protocol, region, "fcu"),
            lbu: default_aws_endpoint(protocol, region, "lbu"),
            eim: default_aws_endpoint(protocol, region, "eim"),
            icu: default_aws_endpoint(protocol, region, "icu"),
            oos: default_aws_endpoint(protocol, region, "oos"),
            oks: default_oks_endpoint(protocol, region),
        }
    }
}

struct EndpointsSeed {
    protocol: String,
    region: String,
}

impl<'de> DeserializeSeed<'de> for EndpointsSeed {
    type Value = Endpoint;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Api,
            Fcu,
            Lbu,
            Eim,
            Icu,
            Oos,
            Oks,
        }

        struct EndpointVisitor {
            protocol: String,
            region: String,
        }

        impl<'de> serde::de::Visitor<'de> for EndpointVisitor {
            type Value = Endpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Endpoint")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut api = None;
                let mut fcu = None;
                let mut lbu = None;
                let mut eim = None;
                let mut icu = None;
                let mut oos = None;
                let mut oks = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::Api => {
                            api = Some(map.next_value::<String>()?);
                        }
                        Field::Fcu => {
                            fcu = Some(map.next_value::<String>()?);
                        }
                        Field::Lbu => {
                            lbu = Some(map.next_value::<String>()?);
                        }
                        Field::Eim => {
                            eim = Some(map.next_value::<String>()?);
                        }
                        Field::Icu => {
                            icu = Some(map.next_value::<String>()?);
                        }
                        Field::Oos => {
                            oos = Some(map.next_value::<String>()?);
                        }
                        Field::Oks => {
                            oks = Some(map.next_value::<String>()?);
                        }
                    }
                }

                let api = api.unwrap_or_else(|| {
                    default_oapi_endpoint(self.protocol.as_str(), self.region.as_str())
                });
                let fcu = fcu.unwrap_or_else(|| {
                    default_aws_endpoint(self.protocol.as_str(), self.region.as_str(), "fcu")
                });
                let lbu = lbu.unwrap_or_else(|| {
                    default_aws_endpoint(self.protocol.as_str(), self.region.as_str(), "lbu")
                });
                let eim = eim.unwrap_or_else(|| {
                    default_aws_endpoint(self.protocol.as_str(), self.region.as_str(), "eim")
                });
                let icu = icu.unwrap_or_else(|| {
                    default_aws_endpoint(self.protocol.as_str(), self.region.as_str(), "icu")
                });
                let oos = oos.unwrap_or_else(|| {
                    default_aws_endpoint(self.protocol.as_str(), self.region.as_str(), "oos")
                });
                let oks = oks.unwrap_or_else(|| {
                    default_oks_endpoint(self.protocol.as_str(), self.region.as_str())
                });

                Ok(Endpoint {
                    api,
                    fcu,
                    lbu,
                    eim,
                    icu,
                    oos,
                    oks,
                })
            }
        }

        let visitor = EndpointVisitor {
            protocol: self.protocol,
            region: self.region,
        };
        deserializer.deserialize_struct(
            "Endpoint",
            &["api", "fcu", "lbu", "eim", "icu", "oos", "oks"],
            visitor,
        )
    }
}

#[derive(Debug)]
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

impl<'de> Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            AccessKey,
            SecretKey,
            X509ClientCert,
            X509ClientKey,
            X509ClientCertB64,
            X509ClientKeyB64,
            TlsSkipVerify,
            Login,
            Password,
            Protocol,
            Region,
            Endpoints,
        }

        struct ProfileVisitor;

        impl<'de> serde::de::Visitor<'de> for ProfileVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Profile")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut access_key = None;
                let mut secret_key = None;
                let mut x509_client_cert = None;
                let mut x509_client_key = None;
                let mut x509_client_cert_b64 = None;
                let mut x509_client_key_b64 = None;
                let mut tls_skip_verify = None;
                let mut login = None;
                let password = None;
                let mut protocol = None;
                let mut region = None;
                let mut endpoints = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        Field::AccessKey => {
                            access_key = Some(map.next_value::<SecretString>()?);
                        }
                        Field::SecretKey => {
                            secret_key = Some(map.next_value::<SecretString>()?);
                        }
                        Field::X509ClientCert => {
                            x509_client_cert = Some(map.next_value::<String>()?);
                        }
                        Field::X509ClientKey => {
                            x509_client_key = Some(map.next_value::<String>()?);
                        }
                        Field::X509ClientCertB64 => {
                            x509_client_cert_b64 = Some(map.next_value::<SecretString>()?);
                        }
                        Field::X509ClientKeyB64 => {
                            x509_client_key_b64 = Some(map.next_value::<SecretString>()?);
                        }
                        Field::TlsSkipVerify => {
                            tls_skip_verify = Some(map.next_value::<bool>()?);
                        }
                        Field::Login => {
                            login = Some(map.next_value::<SecretString>()?);
                        }
                        Field::Password => {
                            secret_key = Some(map.next_value::<SecretString>()?);
                        }
                        Field::Protocol => {
                            protocol = Some(map.next_value::<String>()?);
                        }
                        Field::Region => {
                            region = Some(map.next_value::<String>()?);
                        }
                        _ => {}
                    }
                }

                let region = region.unwrap_or_else(|| "eu-west-2".to_string());
                let protocol = protocol.unwrap_or_else(|| "https".to_string());
                let tls_skip_verify = tls_skip_verify.unwrap_or_default();

                while let Some(field) = map.next_key()? {
                    if let Field::Endpoints = field {
                        endpoints = Some(map.next_value_seed(EndpointsSeed {
                            protocol: protocol.to_string(),
                            region: region.to_string(),
                        })?);
                    }
                }

                let endpoints = endpoints.unwrap_or_else(|| Endpoint::new(&protocol, &region));

                Ok(Profile {
                    access_key,
                    secret_key,
                    x509_client_cert,
                    x509_client_key,
                    x509_client_cert_b64,
                    x509_client_key_b64,
                    tls_skip_verify,
                    login,
                    password,
                    protocol,
                    region,
                    endpoints,
                })
            }
        }

        deserializer.deserialize_struct(
            "Endpoints",
            &[
                "access_key",
                "secret_key",
                "x509_client_cert",
                "x509_client_key",
                "x509_client_cert_b64",
                "x509_client_key_b64",
                "tls_skip_verify",
                "login",
                "password",
                "protocol",
                "region",
                "endpoints",
            ],
            ProfileVisitor,
        )
    }
}

type JsonObject = serde_json::Map<String, serde_json::Value>;

trait Provider {
    fn data(&self, profile: &str) -> Result<JsonObject, super::Error>;

    fn profile(&self) -> Option<String>;
}

#[derive(Debug)]
pub struct FromFile {
    inner: serde_json::Map<String, serde_json::Value>,
}

impl FromFile {
    pub fn new(path: &std::path::Path) -> Result<Self, super::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let inner = serde_json::from_reader(reader)?;
        Ok(FromFile { inner })
    }
}

impl Provider for FromFile {
    fn data(&self, profile: &str) -> Result<JsonObject, super::Error> {
        if profile.is_empty() {
            return Ok(JsonObject::new());
        }

        if let Some(p) = self.inner.get(profile) {
            if let Some(pp) = p.as_object() {
                Ok(pp.clone())
            } else {
                Err(super::Error::UnsupportedFeature(
                    "non object profile".to_string(),
                ))
            }
        } else {
            Err(super::Error::ProfileNotFound(profile.to_string()))
        }
    }

    fn profile(&self) -> Option<String> {
        for (name, profile) in self.inner.iter() {
            if let Some(d) = profile.get("default")
                && d.as_bool().unwrap_or_default()
            {
                return Some(name.to_owned());
            }
        }

        if self.inner.get("default").is_some() {
            return Some("default".to_owned());
        }

        None
    }
}

pub struct FromEnv;

impl Provider for FromEnv {
    fn data(&self, _profile: &str) -> Result<JsonObject, super::Error> {
        let mut data = JsonObject::new();

        for (key, env) in [
            ("access_key", "OSC_ACCESS_KEY"),
            ("secret_key", "OSC_SECRET_KEY"),
            ("x509_client_cert", "OSC_X509_CLENT_CERT"),
            ("x509_client_key", "OSC_X509_CLENT_KEY"),
            ("x509_client_cert_b64", "OSC_X509_CLENT_CERT_B64"),
            ("x509_client_key_b64", "OSC_X509_CLENT_KEY_B64"),
            ("login", "OSC_LOGIN"),
            ("password", "OSC_PASSWORD"),
            ("protocol", "OSC_PROTOCOL"),
            ("region", "OSC_REGION"),
        ] {
            match std::env::var(env) {
                Ok(v) => {
                    data.insert(key.to_string(), serde_json::Value::String(v));
                }
                Err(std::env::VarError::NotPresent) => {}
                Err(e) => {
                    return Err(super::Error::InvalidEnvironmentVariable {
                        name: env.to_string(),
                        reason: Some(e.to_string()),
                    });
                }
            };
        }

        match std::env::var("OSC_TLS_SKIP_VERIFY") {
            Ok(e) if e.to_lowercase() == "true" => {
                data.insert("tls_skip_verify".to_string(), serde_json::Value::Bool(true));
            }
            Ok(_) => {
                data.insert(
                    "tls_skip_verify".to_string(),
                    serde_json::Value::Bool(false),
                );
            }
            Err(std::env::VarError::NotPresent) => {}
            Err(e) => {
                return Err(super::Error::InvalidEnvironmentVariable {
                    name: "OSC_TLS_SKIP_VERIFY".to_string(),
                    reason: Some(e.to_string()),
                });
            }
        }

        let mut endpoint_data = JsonObject::new();

        for (key, env) in [
            ("api", "OSC_ENDPOINT_API"),
            ("fcu", "OSC_ENDPOINT_FCU"),
            ("lbu", "OSC_ENDPOINT_LBU"),
            ("eim", "OSC_ENDPOINT_EIM"),
            ("icu", "OSC_ENDPOINT_ICU"),
            ("oos", "OSC_ENDPOINT_OOS"),
            ("oks", "OSC_ENDPOINT_OKS"),
        ] {
            match std::env::var(env) {
                Ok(v) => {
                    endpoint_data.insert(key.to_string(), serde_json::Value::String(v));
                }
                Err(std::env::VarError::NotPresent) => {}
                Err(e) => {
                    return Err(super::Error::InvalidEnvironmentVariable {
                        name: env.to_string(),
                        reason: Some(e.to_string()),
                    });
                }
            };
        }

        data.insert("endpoints".to_string(), endpoint_data.into());

        Ok(data)
    }

    fn profile(&self) -> Option<String> {
        std::env::var("OSC_PROFILE").ok()
    }
}

pub struct Stack<Inner, Outer> {
    inner: Inner,
    outer: Outer,
}

impl<Inner, Outer> Provider for Stack<Inner, Outer>
where
    Inner: Provider,
    Outer: Provider,
{
    fn data(&self, profile: &str) -> Result<JsonObject, super::Error> {
        let mut inner_data = self.inner.data(profile)?;
        let outer_data = self.outer.data(profile)?;

        inner_data.extend(outer_data);
        Ok(inner_data)
    }

    fn profile(&self) -> Option<String> {
        self.inner.profile().or_else(|| self.outer.profile())
    }
}

fn to_profile(p: impl Provider) -> Result<Profile, super::Error> {
    let profile_name = p.profile().unwrap_or_default();
    let data = p.data(profile_name.as_str())?;

    serde_json::from_value::<Profile>(serde_json::Value::Object(data))
        .map_err(|e| super::Error::ProfileNotFound(e.to_string()))
}

impl Profile {
    pub fn new() -> Result<Self, super::Error> {
        let mut file_provider = None;

        if let Some(mut path) = home::home_dir() {
            path.push(".osc/config.json");

            if path.exists() {
                file_provider = FromFile::new(&path).ok();
            }
        }

        if let Some(file) = file_provider {
            let stack = Stack {
                inner: file,
                outer: FromEnv,
            };

            to_profile(stack)
        } else {
            to_profile(FromEnv)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;
    use serial_test::serial;
    use std::io::Write;

    // ── helpers ──────────────────────────────────────────────────────────────

    /// Write *content* to a named temp-file and return both the handle (kept
    /// alive so the file is not deleted) and the path.
    fn temp_json_file(content: &str) -> (tempfile::NamedTempFile, std::path::PathBuf) {
        let mut f = tempfile::NamedTempFile::new().expect("tmp file");
        f.write_all(content.as_bytes()).expect("write");
        let path = f.path().to_path_buf();
        (f, path)
    }

    // ── default endpoint helpers ──────────────────────────────────────────

    #[test]
    fn test_default_oapi_endpoint() {
        assert_eq!(
            default_oapi_endpoint("https", "eu-west-2"),
            "https://api.eu-west-2.outscale.com/api/v1"
        );
    }

    #[test]
    fn test_default_aws_endpoint() {
        assert_eq!(
            default_aws_endpoint("https", "eu-west-2", "fcu"),
            "https://fcu.eu-west-2.outscale.com"
        );
    }

    #[test]
    fn test_default_oks_endpoint() {
        assert_eq!(
            default_oks_endpoint("https", "eu-west-2"),
            "https://api.eu-west-2.oks.outscale.com/api/v2"
        );
    }

    // ── Endpoint::new ─────────────────────────────────────────────────────

    #[test]
    fn test_endpoint_new_defaults() {
        let ep = Endpoint::new("https", "eu-west-2");
        assert_eq!(ep.api, "https://api.eu-west-2.outscale.com/api/v1");
        assert_eq!(ep.fcu, "https://fcu.eu-west-2.outscale.com");
        assert_eq!(ep.lbu, "https://lbu.eu-west-2.outscale.com");
        assert_eq!(ep.eim, "https://eim.eu-west-2.outscale.com");
        assert_eq!(ep.icu, "https://icu.eu-west-2.outscale.com");
        assert_eq!(ep.oos, "https://oos.eu-west-2.outscale.com");
        assert_eq!(ep.oks, "https://api.eu-west-2.oks.outscale.com/api/v2");
    }

    #[test]
    fn test_endpoint_new_http_protocol() {
        let ep = Endpoint::new("http", "us-east-1");
        assert_eq!(ep.api, "http://api.us-east-1.outscale.com/api/v1");
        assert_eq!(ep.oks, "http://api.us-east-1.oks.outscale.com/api/v2");
    }

    // ── Profile deserialization ───────────────────────────────────────────

    #[test]
    fn test_profile_deserialize_full() {
        let json = serde_json::json!({
            "access_key": "AK123",
            "secret_key": "SK456",
            "region": "eu-west-2",
            "protocol": "https",
            "tls_skip_verify": true,
            "login": "user@example.com",
            "x509_client_cert": "/path/cert.pem",
            "x509_client_key": "/path/key.pem",
            "x509_client_cert_b64": "Y2VydA==",
            "x509_client_key_b64": "a2V5"
        });

        let p: Profile = serde_json::from_value(json).expect("deserialize");
        assert_eq!(p.access_key.unwrap().expose_secret(), "AK123");
        assert_eq!(p.secret_key.unwrap().expose_secret(), "SK456");
        assert_eq!(p.region, "eu-west-2");
        assert_eq!(p.protocol, "https");
        assert!(p.tls_skip_verify);
        assert_eq!(p.login.unwrap().expose_secret(), "user@example.com");
        assert_eq!(p.x509_client_cert.unwrap(), "/path/cert.pem");
        assert_eq!(p.x509_client_key.unwrap(), "/path/key.pem");
        assert_eq!(p.x509_client_cert_b64.unwrap().expose_secret(), "Y2VydA==");
        assert_eq!(p.x509_client_key_b64.unwrap().expose_secret(), "a2V5");
    }

    #[test]
    fn test_profile_deserialize_defaults() {
        // Only the bare minimum – everything else should fall back to defaults
        let json = serde_json::json!({});
        let p: Profile = serde_json::from_value(json).expect("deserialize");
        assert_eq!(p.region, "eu-west-2");
        assert_eq!(p.protocol, "https");
        assert!(!p.tls_skip_verify);
        assert!(p.access_key.is_none());
        assert!(p.secret_key.is_none());
        // Endpoints should be built from the defaults
        assert_eq!(p.endpoints.api, "https://api.eu-west-2.outscale.com/api/v1");
    }

    #[test]
    fn test_profile_deserialize_custom_region_and_protocol() {
        let json = serde_json::json!({
            "region": "ap-northeast-1",
            "protocol": "http"
        });
        let p: Profile = serde_json::from_value(json).expect("deserialize");
        assert_eq!(p.region, "ap-northeast-1");
        assert_eq!(p.protocol, "http");
        assert_eq!(
            p.endpoints.api,
            "http://api.ap-northeast-1.outscale.com/api/v1"
        );
        assert_eq!(p.endpoints.fcu, "http://fcu.ap-northeast-1.outscale.com");
    }

    #[test]
    fn test_profile_tls_skip_verify_false() {
        let json = serde_json::json!({ "tls_skip_verify": false });
        let p: Profile = serde_json::from_value(json).expect("deserialize");
        assert!(!p.tls_skip_verify);
    }

    /// `password` field is deserialized into `secret_key` by the current impl.
    #[test]
    fn test_profile_password_maps_to_secret_key() {
        let json = serde_json::json!({ "password": "mysecret" });
        let p: Profile = serde_json::from_value(json).expect("deserialize");
        assert_eq!(p.secret_key.unwrap().expose_secret(), "mysecret");
        assert!(p.password.is_none());
    }

    // ── EndpointsSeed / Endpoint deserialization ──────────────────────────

    #[test]
    fn test_endpoints_seed_all_overrides() {
        let json = serde_json::json!({
            "api": "https://custom.api.example.com",
            "fcu": "https://custom.fcu.example.com",
            "lbu": "https://custom.lbu.example.com",
            "eim": "https://custom.eim.example.com",
            "icu": "https://custom.icu.example.com",
            "oos": "https://custom.oos.example.com",
            "oks": "https://custom.oks.example.com"
        });

        let seed = EndpointsSeed {
            protocol: "https".to_string(),
            region: "eu-west-2".to_string(),
        };
        let ep: Endpoint = seed.deserialize(json).expect("deserialize endpoints");

        assert_eq!(ep.api, "https://custom.api.example.com");
        assert_eq!(ep.fcu, "https://custom.fcu.example.com");
        assert_eq!(ep.lbu, "https://custom.lbu.example.com");
        assert_eq!(ep.eim, "https://custom.eim.example.com");
        assert_eq!(ep.icu, "https://custom.icu.example.com");
        assert_eq!(ep.oos, "https://custom.oos.example.com");
        assert_eq!(ep.oks, "https://custom.oks.example.com");
    }

    #[test]
    fn test_endpoints_seed_partial_override_falls_back_to_default() {
        let json = serde_json::json!({ "api": "https://my.api.example.com" });

        let seed = EndpointsSeed {
            protocol: "https".to_string(),
            region: "eu-west-2".to_string(),
        };
        let ep: Endpoint = seed.deserialize(json).expect("deserialize endpoints");

        assert_eq!(ep.api, "https://my.api.example.com");
        // All others must be the computed defaults
        assert_eq!(ep.fcu, "https://fcu.eu-west-2.outscale.com");
        assert_eq!(ep.lbu, "https://lbu.eu-west-2.outscale.com");
        assert_eq!(ep.eim, "https://eim.eu-west-2.outscale.com");
        assert_eq!(ep.icu, "https://icu.eu-west-2.outscale.com");
        assert_eq!(ep.oos, "https://oos.eu-west-2.outscale.com");
        assert_eq!(ep.oks, "https://api.eu-west-2.oks.outscale.com/api/v2");
    }

    #[test]
    fn test_endpoints_seed_empty_uses_all_defaults() {
        let json = serde_json::json!({});
        let seed = EndpointsSeed {
            protocol: "https".to_string(),
            region: "eu-west-2".to_string(),
        };
        let ep: Endpoint = seed.deserialize(json).expect("deserialize endpoints");
        let expected = Endpoint::new("https", "eu-west-2");
        assert_eq!(ep.api, expected.api);
        assert_eq!(ep.fcu, expected.fcu);
        assert_eq!(ep.lbu, expected.lbu);
        assert_eq!(ep.eim, expected.eim);
        assert_eq!(ep.icu, expected.icu);
        assert_eq!(ep.oos, expected.oos);
        assert_eq!(ep.oks, expected.oks);
    }

    // ── FromFile provider ─────────────────────────────────────────────────

    #[test]
    fn test_fromfile_data_returns_profile_object() {
        let content = r#"{"myprofile": {"access_key": "AK", "secret_key": "SK"}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        let data = provider.data("myprofile").expect("data");
        assert_eq!(data.get("access_key").and_then(|v| v.as_str()), Some("AK"));
    }

    #[test]
    fn test_fromfile_profile_not_found() {
        let content = r#"{"other": {}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        let err = provider.data("missing").unwrap_err();
        assert!(
            matches!(err, super::super::Error::ProfileNotFound(ref n) if n == "missing"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_fromfile_empty_profile_name_returns_empty_map() {
        let content = r#"{"myprofile": {"access_key": "AK"}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        let data = provider.data("").expect("data");
        assert!(data.is_empty());
    }

    #[test]
    fn test_fromfile_non_object_profile_returns_error() {
        let content = r#"{"bad": "not an object"}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        let err = provider.data("bad").unwrap_err();
        assert!(
            matches!(err, super::super::Error::UnsupportedFeature(_)),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_fromfile_profile_auto_detects_default_key() {
        // A profile named "default" is returned as the active profile
        let content = r#"{"default": {"access_key": "AK"}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        assert_eq!(provider.profile(), Some("default".to_string()));
    }

    #[test]
    fn test_fromfile_profile_auto_detects_default_flag() {
        let content = r#"{"prod": {"default": true, "access_key": "AK"}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        assert_eq!(provider.profile(), Some("prod".to_string()));
    }

    #[test]
    fn test_fromfile_profile_returns_none_when_no_default() {
        let content = r#"{"prod": {"access_key": "AK"}}"#;
        let (_f, path) = temp_json_file(content);
        let provider = FromFile::new(&path).expect("FromFile::new");
        assert_eq!(provider.profile(), None);
    }

    #[test]
    fn test_fromfile_new_missing_file_returns_io_error() {
        let err = FromFile::new(std::path::Path::new("/nonexistent/path/config.json")).unwrap_err();
        assert!(matches!(err, super::super::Error::Io(_)));
    }

    #[test]
    fn test_fromfile_new_invalid_json_returns_serialization_error() {
        let (_f, path) = temp_json_file("not valid json {{{");
        let err = FromFile::new(&path).unwrap_err();
        assert!(matches!(
            err,
            super::super::Error::Serialization(super::super::errors::SerializationError::Json(_))
        ));
    }

    // ── FromEnv provider ──────────────────────────────────────────────────

    /// Guard that clears a list of env vars on drop to avoid test pollution.
    struct EnvGuard(Vec<&'static str>);

    impl EnvGuard {
        fn set(vars: &[(&'static str, &str)]) -> Self {
            for (k, v) in vars {
                // SAFETY: tests run with RUST_TEST_THREADS=1; no other thread
                // reads the environment concurrently within the test suite.
                unsafe { std::env::set_var(k, v) };
            }
            Self(vars.iter().map(|(k, _)| *k).collect())
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for k in &self.0 {
                unsafe { std::env::remove_var(k) };
            }
        }
    }

    #[test]
    #[serial]
    fn test_fromenv_data_basic_vars() {
        let _g = EnvGuard::set(&[
            ("OSC_ACCESS_KEY", "ENVAK"),
            ("OSC_SECRET_KEY", "ENVSK"),
            ("OSC_REGION", "eu-west-2"),
        ]);

        let data = FromEnv.data("any").expect("data");
        assert_eq!(
            data.get("access_key").and_then(|v| v.as_str()),
            Some("ENVAK")
        );
        assert_eq!(
            data.get("secret_key").and_then(|v| v.as_str()),
            Some("ENVSK")
        );
        assert_eq!(
            data.get("region").and_then(|v| v.as_str()),
            Some("eu-west-2")
        );
    }

    #[test]
    #[serial]
    fn test_fromenv_data_tls_skip_verify_true() {
        let _g = EnvGuard::set(&[("OSC_TLS_SKIP_VERIFY", "true")]);
        let data = FromEnv.data("any").expect("data");
        assert_eq!(
            data.get("tls_skip_verify").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    #[serial]
    fn test_fromenv_data_tls_skip_verify_true_case_insensitive() {
        let _g = EnvGuard::set(&[("OSC_TLS_SKIP_VERIFY", "TRUE")]);
        let data = FromEnv.data("any").expect("data");
        assert_eq!(
            data.get("tls_skip_verify").and_then(|v| v.as_bool()),
            Some(true)
        );
    }

    #[test]
    #[serial]
    fn test_fromenv_data_tls_skip_verify_non_true_is_false() {
        let _g = EnvGuard::set(&[("OSC_TLS_SKIP_VERIFY", "yes")]);
        let data = FromEnv.data("any").expect("data");
        assert_eq!(
            data.get("tls_skip_verify").and_then(|v| v.as_bool()),
            Some(false)
        );
    }

    #[test]
    #[serial]
    fn test_fromenv_data_endpoint_vars() {
        let _g = EnvGuard::set(&[
            ("OSC_ENDPOINT_API", "https://custom.api"),
            ("OSC_ENDPOINT_FCU", "https://custom.fcu"),
        ]);
        let data = FromEnv.data("any").expect("data");
        let endpoints = data
            .get("endpoints")
            .and_then(|v| v.as_object())
            .expect("endpoints");
        assert_eq!(
            endpoints.get("api").and_then(|v| v.as_str()),
            Some("https://custom.api")
        );
        assert_eq!(
            endpoints.get("fcu").and_then(|v| v.as_str()),
            Some("https://custom.fcu")
        );
        // lbu was not set – must be absent from the map (not a default string)
        assert!(endpoints.get("lbu").is_none());
    }

    #[test]
    #[serial]
    fn test_fromenv_data_empty_when_no_vars_set() {
        // Remove any env vars that might have been set by a prior test
        let keys = [
            "OSC_ACCESS_KEY",
            "OSC_SECRET_KEY",
            "OSC_REGION",
            "OSC_PROTOCOL",
            "OSC_TLS_SKIP_VERIFY",
            "OSC_LOGIN",
            "OSC_PASSWORD",
            "OSC_X509_CLENT_CERT",
            "OSC_X509_CLENT_KEY",
            "OSC_X509_CLENT_CERT_B64",
            "OSC_X509_CLENT_KEY_B64",
            "OSC_ENDPOINT_API",
            "OSC_ENDPOINT_FCU",
            "OSC_ENDPOINT_LBU",
            "OSC_ENDPOINT_EIM",
            "OSC_ENDPOINT_ICU",
            "OSC_ENDPOINT_OOS",
            "OSC_ENDPOINT_OKS",
        ];
        for k in keys {
            unsafe { std::env::remove_var(k) };
        }

        let data = FromEnv.data("any").expect("data");
        // Only "endpoints" key should be present (as an empty object)
        assert!(data.get("access_key").is_none());
        assert!(data.get("secret_key").is_none());
        assert!(data.get("tls_skip_verify").is_none());
        let endpoints = data
            .get("endpoints")
            .and_then(|v| v.as_object())
            .expect("endpoints key");
        assert!(endpoints.is_empty());
    }

    #[test]
    #[serial]
    fn test_fromenv_profile_returns_osc_profile_var() {
        let _g = EnvGuard::set(&[("OSC_PROFILE", "staging")]);
        assert_eq!(FromEnv.profile(), Some("staging".to_string()));
    }

    #[test]
    #[serial]
    fn test_fromenv_profile_returns_none_when_unset() {
        unsafe { std::env::remove_var("OSC_PROFILE") };
        assert_eq!(FromEnv.profile(), None);
    }

    // ── Stack provider ────────────────────────────────────────────────────

    /// Minimal in-memory provider for Stack tests.
    struct MapProvider {
        data: JsonObject,
        profile: Option<String>,
    }

    impl Provider for MapProvider {
        fn data(&self, _profile: &str) -> Result<JsonObject, super::super::Error> {
            Ok(self.data.clone())
        }
        fn profile(&self) -> Option<String> {
            self.profile.clone()
        }
    }

    fn map_provider(pairs: &[(&str, &str)], profile: Option<&str>) -> MapProvider {
        let mut data = JsonObject::new();
        for (k, v) in pairs {
            data.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
        MapProvider {
            data,
            profile: profile.map(String::from),
        }
    }

    #[test]
    fn test_stack_outer_overrides_inner() {
        let inner = map_provider(&[("key", "from_inner"), ("only_inner", "yes")], None);
        let outer = map_provider(&[("key", "from_outer")], None);
        let stack = Stack { inner, outer };

        let data = stack.data("").expect("data");
        assert_eq!(data.get("key").and_then(|v| v.as_str()), Some("from_outer"));
        assert_eq!(data.get("only_inner").and_then(|v| v.as_str()), Some("yes"));
    }

    #[test]
    fn test_stack_profile_prefers_inner() {
        let inner = map_provider(&[], Some("inner_profile"));
        let outer = map_provider(&[], Some("outer_profile"));
        let stack = Stack { inner, outer };
        assert_eq!(stack.profile(), Some("inner_profile".to_string()));
    }

    #[test]
    fn test_stack_profile_falls_back_to_outer_when_inner_is_none() {
        let inner = map_provider(&[], None);
        let outer = map_provider(&[], Some("outer_profile"));
        let stack = Stack { inner, outer };
        assert_eq!(stack.profile(), Some("outer_profile".to_string()));
    }

    #[test]
    fn test_stack_profile_none_when_both_none() {
        let inner = map_provider(&[], None);
        let outer = map_provider(&[], None);
        let stack = Stack { inner, outer };
        assert_eq!(stack.profile(), None);
    }

    #[test]
    fn test_stack_propagates_inner_error() {
        struct ErrProvider;
        impl Provider for ErrProvider {
            fn data(&self, _: &str) -> Result<JsonObject, super::super::Error> {
                Err(super::super::Error::ProfileNotFound("boom".to_string()))
            }
            fn profile(&self) -> Option<String> {
                None
            }
        }

        let stack = Stack {
            inner: ErrProvider,
            outer: map_provider(&[], None),
        };
        assert!(stack.data("").is_err());
    }
}
