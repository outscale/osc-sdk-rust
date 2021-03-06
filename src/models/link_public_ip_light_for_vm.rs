/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkPublicIpLightForVm : Information about the public IP associated with the NIC.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkPublicIpLightForVm {
    /// The name of the public DNS.
    #[serde(rename = "PublicDnsName", skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// The public IP associated with the NIC.
    #[serde(rename = "PublicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// The account ID of the owner of the public IP.
    #[serde(rename = "PublicIpAccountId", skip_serializing_if = "Option::is_none")]
    pub public_ip_account_id: Option<String>,
}

impl LinkPublicIpLightForVm {
    /// Information about the public IP associated with the NIC.
    pub fn new() -> LinkPublicIpLightForVm {
        LinkPublicIpLightForVm {
            public_dns_name: None,
            public_ip: None,
            public_ip_account_id: None,
        }
    }
}
