/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// PrivateIpLightForVm : Information about the private IP of the NIC.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrivateIpLightForVm {
    /// If true, the IP is the primary private IP of the NIC.
    #[serde(rename = "IsPrimary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "LinkPublicIp", skip_serializing_if = "Option::is_none")]
    pub link_public_ip: Option<Box<crate::models::LinkPublicIpLightForVm>>,
    /// The name of the private DNS.
    #[serde(rename = "PrivateDnsName", skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// The private IP.
    #[serde(rename = "PrivateIp", skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
}

impl PrivateIpLightForVm {
    /// Information about the private IP of the NIC.
    pub fn new() -> PrivateIpLightForVm {
        PrivateIpLightForVm {
            is_primary: None,
            link_public_ip: None,
            private_dns_name: None,
            private_ip: None,
        }
    }
}
