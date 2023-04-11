/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiAccessPolicy : Information about the API access policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiAccessPolicy {
    /// The maximum possible lifetime for your access keys, in seconds. If `0`, your access keys can have unlimited lifetimes.
    #[serde(
        rename = "MaxAccessKeyExpirationSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_access_key_expiration_seconds: Option<i64>,
    /// If true, a trusted session is activated, allowing you to bypass Certificate Authorities (CAs) enforcement. For more information, see the `ApiKeyAuth` authentication scheme in the [Authentication](#authentication) section.
    #[serde(rename = "RequireTrustedEnv", skip_serializing_if = "Option::is_none")]
    pub require_trusted_env: Option<bool>,
}

impl ApiAccessPolicy {
    /// Information about the API access policy.
    pub fn new() -> ApiAccessPolicy {
        ApiAccessPolicy {
            max_access_key_expiration_seconds: None,
            require_trusted_env: None,
        }
    }
}
