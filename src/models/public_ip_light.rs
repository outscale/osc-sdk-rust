/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// PublicIpLight : Information about the public IP.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublicIpLight {
    /// The public IP associated with the NAT service.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// The allocation ID of the public IP associated with the NAT service.
    #[serde(rename = "PublicIpId", skip_serializing_if = "Option::is_none")]
    pub public_ip_id: Option<String>,
}

impl PublicIpLight {
    /// Information about the public IP.
    pub fn new() -> PublicIpLight {
        PublicIpLight {
            public_ip: None,
            public_ip_id: None,
        }
    }
}
