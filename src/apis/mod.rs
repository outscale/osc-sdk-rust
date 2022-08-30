use aws_sigv4;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
    AWSV4SignatureError(aws_sigv4::http_request::Error),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
            Error::AWSV4SignatureError(e) => ("aws v4 signature", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
            Error::AWSV4SignatureError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod access_key_api;
pub mod account_api;
pub mod api_access_policy_api;
pub mod api_access_rule_api;
pub mod api_log_api;
pub mod ca_api;
pub mod catalog_api;
pub mod client_gateway_api;
pub mod configuration;
pub mod configuration_file;
pub mod dhcp_option_api;
pub mod direct_link_api;
pub mod direct_link_interface_api;
pub mod flexible_gpu_api;
pub mod image_api;
pub mod internet_service_api;
pub mod keypair_api;
pub mod listener_api;
pub mod load_balancer_api;
pub mod load_balancer_policy_api;
pub mod location_api;
pub mod nat_service_api;
pub mod net_access_point_api;
pub mod net_api;
pub mod net_peering_api;
pub mod nic_api;
pub mod product_type_api;
pub mod public_catalog_api;
pub mod public_ip_api;
pub mod quota_api;
pub mod region_api;
pub mod route_api;
pub mod route_table_api;
pub mod security_group_api;
pub mod security_group_rule_api;
pub mod server_certificate_api;
pub mod snapshot_api;
pub mod subnet_api;
pub mod subregion_api;
pub mod tag_api;
pub mod task_api;
pub mod virtual_gateway_api;
pub mod vm_api;
pub mod volume_api;
pub mod vpn_connection_api;
