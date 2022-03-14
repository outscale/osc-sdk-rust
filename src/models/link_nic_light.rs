/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkNicLight : Information about the network interface card (NIC).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkNicLight {
    /// If true, the NIC is deleted when the VM is terminated.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The device index for the NIC attachment (between 1 and 7, both included).
    #[serde(rename = "DeviceNumber", skip_serializing_if = "Option::is_none")]
    pub device_number: Option<i32>,
    /// The ID of the NIC to attach.
    #[serde(rename = "LinkNicId", skip_serializing_if = "Option::is_none")]
    pub link_nic_id: Option<String>,
    /// The state of the attachment (`attaching` \\| `attached` \\| `detaching` \\| `detached`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl LinkNicLight {
    /// Information about the network interface card (NIC).
    pub fn new() -> LinkNicLight {
        LinkNicLight {
            delete_on_vm_deletion: None,
            device_number: None,
            link_nic_id: None,
            state: None,
        }
    }
}
