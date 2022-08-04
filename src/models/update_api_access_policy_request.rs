/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateApiAccessPolicyRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The maximum possible lifetime for your access keys, in seconds (between `0` and `3153600000`, both included). If set to `O`, your access keys can have unlimited lifetimes, but a trusted session cannot be activated. Otherwise, all your access keys must have an expiration date. This value must be greater than the remaining lifetime of each access key of your account.
    #[serde(rename = "MaxAccessKeyExpirationSeconds")]
    pub max_access_key_expiration_seconds: i64,
    /// If true, a trusted session is activated, provided that you specify the `MaxAccessKeyExpirationSeconds` parameter with a value greater than `0`.
    #[serde(rename = "RequireTrustedEnv")]
    pub require_trusted_env: bool,
}

impl UpdateApiAccessPolicyRequest {
    pub fn new(
        max_access_key_expiration_seconds: i64,
        require_trusted_env: bool,
    ) -> UpdateApiAccessPolicyRequest {
        UpdateApiAccessPolicyRequest {
            dry_run: None,
            max_access_key_expiration_seconds,
            require_trusted_env,
        }
    }
}
