/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkPrivateIpsRequest {
    /// If true, allows an IP that is already assigned to another NIC in the same Subnet to be assigned to the NIC you specified.
    #[serde(rename = "AllowRelink", skip_serializing_if = "Option::is_none")]
    pub allow_relink: Option<bool>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the NIC.
    #[serde(rename = "NicId")]
    pub nic_id: String,
    /// The secondary private IP or IPs you want to assign to the NIC within the IP range of the Subnet.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<String>>,
    /// The number of secondary private IPs to assign to the NIC.
    #[serde(
        rename = "SecondaryPrivateIpCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_private_ip_count: Option<i32>,
}

impl LinkPrivateIpsRequest {
    pub fn new(nic_id: String) -> LinkPrivateIpsRequest {
        LinkPrivateIpsRequest {
            allow_relink: None,
            dry_run: None,
            nic_id,
            private_ips: None,
            secondary_private_ip_count: None,
        }
    }
}
