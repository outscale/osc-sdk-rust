/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LoadBalancerTag : Information about the load balancer tag.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoadBalancerTag {
    /// The key of the tag.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the load balancer.
    #[serde(rename = "LoadBalancerName", skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// The value of the tag.
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LoadBalancerTag {
    /// Information about the load balancer tag.
    pub fn new() -> LoadBalancerTag {
        LoadBalancerTag {
            key: None,
            load_balancer_name: None,
            value: None,
        }
    }
}
