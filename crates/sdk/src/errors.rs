use std::fmt::Debug;

use hmac::digest::InvalidLength;
use reqwest::header::InvalidHeaderValue;
use tower::retry::backoff::InvalidBackoff;

#[derive(Debug)]
pub enum Error<T = ()> {
    /// HTTP client errors (includes connection, certificate, timeout errors)
    Http(reqwest::Error),
    /// Application-specific API errors
    Applicative(T),
    /// Data serialization/deserialization errors
    Serialization(SerializationError),
    /// Invalid configuration or environment
    InvalidBaseUrl,
    ProfileNotFound(String),
    Io(std::io::Error),
    InvalidEnvironmentVariable {
        name: String,
        reason: Option<String>,
    },
    UnsupportedFeature(String),
    InvalidClientCertificate(String),
    InvalidBackoff(InvalidBackoff),
    InvalidHeaderValue(InvalidHeaderValue),
    InvalidKeyLength(InvalidLength),
}

#[derive(Debug)]
pub enum SerializationError {
    UrlEncoded(serde_urlencoded::ser::Error),
    Json(serde_json::Error),
    Base64(base64::DecodeError),
}

impl<T> ::std::fmt::Display for Error<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Applicative(_) => write!(f, "oapi error"),
            Error::Serialization(e) => write!(f, "Serialization error: {}", e),
            Error::InvalidBaseUrl => write!(f, "Invalid Base Url"),
            Error::ProfileNotFound(name) => write!(f, "profile not found: {}", name),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::InvalidEnvironmentVariable { name, reason } => {
                write!(f, "invalid environment variable: {}", name)?;
                if let Some(r) = reason {
                    write!(f, " ({})", r)?;
                }
                Ok(())
            }
            Error::UnsupportedFeature(e) => write!(f, "unsupported feature: {}", e),
            Error::InvalidClientCertificate(e) => write!(f, "Invalid client certificate: {}", e),
            Error::InvalidBackoff(e) => write!(f, "Invalid backoff configuration: {}", e),
            Error::InvalidHeaderValue(e) => write!(f, "Invalid header value: {}", e),
            Error::InvalidKeyLength(e) => {
                write!(f, "Invalid key length (is your secret key valid ?): {}", e)
            }
        }
    }
}

impl ::std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            SerializationError::UrlEncoded(e) => write!(f, "url encoding error: {}", e),
            SerializationError::Json(e) => write!(f, "JSON error: {}", e),
            SerializationError::Base64(e) => write!(f, "Base64 error: {}", e),
        }
    }
}

impl<T: Debug> ::std::error::Error for Error<T> {}

impl<T> From<::reqwest::Error> for Error<T> {
    fn from(value: ::reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl<T> From<serde_urlencoded::ser::Error> for Error<T> {
    fn from(value: serde_urlencoded::ser::Error) -> Self {
        Self::Serialization(SerializationError::UrlEncoded(value))
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(SerializationError::Json(e))
    }
}

impl<T> From<base64::DecodeError> for Error<T> {
    fn from(e: base64::DecodeError) -> Self {
        Self::Serialization(SerializationError::Base64(e))
    }
}

impl<T> From<InvalidBackoff> for Error<T> {
    fn from(e: InvalidBackoff) -> Self {
        Self::InvalidBackoff(e)
    }
}

impl<T> From<InvalidHeaderValue> for Error<T> {
    fn from(e: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue(e)
    }
}

impl<T> From<InvalidLength> for Error<T> {
    fn from(e: InvalidLength) -> Self {
        Self::InvalidKeyLength(e)
    }
}
