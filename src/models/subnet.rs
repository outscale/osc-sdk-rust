/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Subnet : Information about the Subnet.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subnet {
    /// The number of available IPs in the Subnets.
    #[serde(rename = "AvailableIpsCount", skip_serializing_if = "Option::is_none")]
    pub available_ips_count: Option<i32>,
    /// The IP range in the Subnet, in CIDR notation (for example, `10.0.0.0/16`).
    #[serde(rename = "IpRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// If true, a public IP is assigned to the network interface cards (NICs) created in the specified Subnet.
    #[serde(
        rename = "MapPublicIpOnLaunch",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_public_ip_on_launch: Option<bool>,
    /// The ID of the Net in which the Subnet is.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The state of the Subnet (`pending` \\| `available` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the Subnet.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// The name of the Subregion in which the Subnet is located.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// One or more tags associated with the Subnet.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl Subnet {
    /// Information about the Subnet.
    pub fn new() -> Subnet {
        Subnet {
            available_ips_count: None,
            ip_range: None,
            map_public_ip_on_launch: None,
            net_id: None,
            state: None,
            subnet_id: None,
            subregion_name: None,
            tags: None,
        }
    }
}
