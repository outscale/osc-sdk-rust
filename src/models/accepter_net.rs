/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// AccepterNet : Information about the accepter Net.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccepterNet {
    /// The account ID of the owner of the accepter Net.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The IP range for the accepter Net, in CIDR notation (for example, `10.0.0.0/16`).
    #[serde(rename = "IpRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// The ID of the accepter Net.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
}

impl AccepterNet {
    /// Information about the accepter Net.
    pub fn new() -> AccepterNet {
        AccepterNet {
            account_id: None,
            ip_range: None,
            net_id: None,
        }
    }
}
