/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkNic : Information about the NIC attachment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkNic {
    /// If true, the NIC is deleted when the VM is terminated.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The device index for the NIC attachment (between `1` and `7`, both included).
    #[serde(rename = "DeviceNumber", skip_serializing_if = "Option::is_none")]
    pub device_number: Option<i32>,
    /// The ID of the NIC to attach.
    #[serde(rename = "LinkNicId", skip_serializing_if = "Option::is_none")]
    pub link_nic_id: Option<String>,
    /// The state of the attachment (`attaching` \\| `attached` \\| `detaching` \\| `detached`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The account ID of the owner of the VM.
    #[serde(rename = "VmAccountId", skip_serializing_if = "Option::is_none")]
    pub vm_account_id: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl LinkNic {
    /// Information about the NIC attachment.
    pub fn new() -> LinkNic {
        LinkNic {
            delete_on_vm_deletion: None,
            device_number: None,
            link_nic_id: None,
            state: None,
            vm_account_id: None,
            vm_id: None,
        }
    }
}
