/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use aws_sigv4::http_request::{sign, SignableRequest, SigningParams, SigningSettings};
use http;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    pub aws_v4_key: Option<AWSv4Key>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct AWSv4Key {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub service: String,
}

impl AWSv4Key {
    pub fn sign(&self, uri: &str, method: &str, body: &str) -> Vec<(String, String)> {
        let request = http::Request::builder()
            .uri(uri)
            .method(method)
            .body(body)
            .unwrap();
        let signing_settings = SigningSettings::default();
        let signing_params = SigningParams::builder()
            .access_key(self.access_key.as_str())
            .secret_key(self.secret_key.as_str())
            .region(self.region.as_str())
            .service_name(self.service.as_str())
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .unwrap();
        let signable_request = SignableRequest::from(&request);
        let (mut signing_instructions, _signature) = sign(signable_request, &signing_params)
            .unwrap()
            .into_parts();
        let mut additional_headers = Vec::<(String, String)>::new();
        if let Some(new_headers) = signing_instructions.take_headers() {
            for (name, value) in new_headers.into_iter() {
                additional_headers.push((
                    name.unwrap().to_string(),
                    value.to_str().unwrap().to_string(),
                ));
            }
        }
        return additional_headers;
    }
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.eu-west-2.outscale.com/api/v1".to_owned(),
            user_agent: Some("osc-sdk-rust/0.3.0".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
            aws_v4_key: None,
        }
    }
}
