/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ApplicationStickyCookiePolicy : Information about the stickiness policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationStickyCookiePolicy {
    /// The name of the application cookie used for stickiness.
    #[serde(rename = "CookieName", skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    /// The mnemonic name for the policy being created. The name must be unique within a set of policies for this load balancer.
    #[serde(rename = "PolicyName", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl ApplicationStickyCookiePolicy {
    /// Information about the stickiness policy.
    pub fn new() -> ApplicationStickyCookiePolicy {
        ApplicationStickyCookiePolicy {
            cookie_name: None,
            policy_name: None,
        }
    }
}
