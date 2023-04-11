/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// NicForVmCreation : Information about the network interface card (NIC) when creating a virtual machine (VM).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NicForVmCreation {
    /// If true, the NIC is deleted when the VM is terminated. You can specify this parameter only for a new NIC. To modify this value for an existing NIC, see [UpdateNic](#updatenic).
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The description of the NIC, if you are creating a NIC when creating the VM.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The index of the VM device for the NIC attachment (between `0` and `7`, both included). This parameter is required if you create a NIC when creating the VM.
    #[serde(rename = "DeviceNumber", skip_serializing_if = "Option::is_none")]
    pub device_number: Option<i32>,
    /// The ID of the NIC, if you are attaching an existing NIC when creating a VM.
    #[serde(rename = "NicId", skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    /// One or more private IPs to assign to the NIC, if you create a NIC when creating a VM. Only one private IP can be the primary private IP.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<crate::models::PrivateIpLight>>,
    /// The number of secondary private IPs, if you create a NIC when creating a VM. This parameter cannot be specified if you specified more than one private IP in the `PrivateIps` parameter.
    #[serde(
        rename = "SecondaryPrivateIpCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_private_ip_count: Option<i32>,
    /// One or more IDs of security groups for the NIC, if you create a NIC when creating a VM.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The ID of the Subnet for the NIC, if you create a NIC when creating a VM. This parameter is required if you create a NIC when creating the VM.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl NicForVmCreation {
    /// Information about the network interface card (NIC) when creating a virtual machine (VM).
    pub fn new() -> NicForVmCreation {
        NicForVmCreation {
            delete_on_vm_deletion: None,
            description: None,
            device_number: None,
            nic_id: None,
            private_ips: None,
            secondary_private_ip_count: None,
            security_group_ids: None,
            subnet_id: None,
        }
    }
}
