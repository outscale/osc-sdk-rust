/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// NicLight : Information about the network interface card (NIC).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NicLight {
    /// The account ID of the owner of the NIC.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The description of the NIC.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// (Net only) If true, the source/destination check is enabled. If false, it is disabled. This value must be false for a NAT VM to perform network address translation (NAT) in a Net.
    #[serde(
        rename = "IsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_dest_checked: Option<bool>,
    #[serde(rename = "LinkNic", skip_serializing_if = "Option::is_none")]
    pub link_nic: Option<Box<crate::models::LinkNicLight>>,
    #[serde(rename = "LinkPublicIp", skip_serializing_if = "Option::is_none")]
    pub link_public_ip: Option<Box<crate::models::LinkPublicIpLightForVm>>,
    /// The Media Access Control (MAC) address of the NIC.
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// The ID of the Net for the NIC.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The ID of the NIC.
    #[serde(rename = "NicId", skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    /// The name of the private DNS.
    #[serde(rename = "PrivateDnsName", skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// The private IP or IPs of the NIC.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<crate::models::PrivateIpLightForVm>>,
    /// One or more IDs of security groups for the NIC.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<crate::models::SecurityGroupLight>>,
    /// The state of the NIC (`available` \\| `attaching` \\| `in-use` \\| `detaching`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the Subnet for the NIC.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl NicLight {
    /// Information about the network interface card (NIC).
    pub fn new() -> NicLight {
        NicLight {
            account_id: None,
            description: None,
            is_source_dest_checked: None,
            link_nic: None,
            link_public_ip: None,
            mac_address: None,
            net_id: None,
            nic_id: None,
            private_dns_name: None,
            private_ips: None,
            security_groups: None,
            state: None,
            subnet_id: None,
        }
    }
}
