/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ResourceLoadBalancerTag : Information about the tag.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResourceLoadBalancerTag {
    /// The key of the tag, with a minimum of 1 character.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl ResourceLoadBalancerTag {
    /// Information about the tag.
    pub fn new() -> ResourceLoadBalancerTag {
        ResourceLoadBalancerTag { key: None }
    }
}
