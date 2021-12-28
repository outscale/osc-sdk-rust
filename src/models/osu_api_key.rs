/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// OsuApiKey : Information about the OOS API key.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OsuApiKey {
    /// The API key of the OOS account that enables you to access the bucket.
    #[serde(rename = "ApiKeyId", skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
    /// The secret key of the OOS account that enables you to access the bucket.
    #[serde(rename = "SecretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

impl OsuApiKey {
    /// Information about the OOS API key.
    pub fn new() -> OsuApiKey {
        OsuApiKey {
            api_key_id: None,
            secret_key: None,
        }
    }
}
