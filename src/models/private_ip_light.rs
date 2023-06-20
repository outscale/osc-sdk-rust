/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// PrivateIpLight : Information about the private IP.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PrivateIpLight {
    /// If true, the IP is the primary private IP of the NIC.
    #[serde(rename = "IsPrimary", skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    /// The private IP of the NIC.
    #[serde(rename = "PrivateIp", skip_serializing_if = "Option::is_none")]
    pub private_ip: Option<String>,
}

impl PrivateIpLight {
    /// Information about the private IP.
    pub fn new() -> PrivateIpLight {
        PrivateIpLight {
            is_primary: None,
            private_ip: None,
        }
    }
}
