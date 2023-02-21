/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Net : Information about the Net.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Net {
    /// The ID of the DHCP options set (or `default` if you want to associate the default one).
    #[serde(rename = "DhcpOptionsSetId", skip_serializing_if = "Option::is_none")]
    pub dhcp_options_set_id: Option<String>,
    /// The IP range for the Net, in CIDR notation (for example, `10.0.0.0/16`).
    #[serde(rename = "IpRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// The ID of the Net.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The state of the Net (`pending` \\| `available` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the Net.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The VM tenancy in a Net.
    #[serde(rename = "Tenancy", skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
}

impl Net {
    /// Information about the Net.
    pub fn new() -> Net {
        Net {
            dhcp_options_set_id: None,
            ip_range: None,
            net_id: None,
            state: None,
            tags: None,
            tenancy: None,
        }
    }
}
