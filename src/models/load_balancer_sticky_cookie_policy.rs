/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerStickyCookiePolicy : Information about the stickiness policy.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerStickyCookiePolicy {
    /// The time period, in seconds, after which the cookie should be considered stale.<br /> If `1`, the stickiness session lasts for the duration of the browser session.
    #[serde(
        rename = "CookieExpirationPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_expiration_period: Option<i32>,
    /// The name of the stickiness policy.
    #[serde(rename = "PolicyName", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

impl LoadBalancerStickyCookiePolicy {
    /// Information about the stickiness policy.
    pub fn new() -> LoadBalancerStickyCookiePolicy {
        LoadBalancerStickyCookiePolicy {
            cookie_expiration_period: None,
            policy_name: None,
        }
    }
}
