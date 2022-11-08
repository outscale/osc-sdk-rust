/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkNicToUpdate : Information about the NIC attachment. If you are modifying the `DeleteOnVmDeletion` attribute, you must specify the ID of the NIC attachment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkNicToUpdate {
    /// If true, the NIC is deleted when the VM is terminated. If false, the NIC is detached from the VM.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The ID of the NIC attachment.
    #[serde(rename = "LinkNicId", skip_serializing_if = "Option::is_none")]
    pub link_nic_id: Option<String>,
}

impl LinkNicToUpdate {
    /// Information about the NIC attachment. If you are modifying the `DeleteOnVmDeletion` attribute, you must specify the ID of the NIC attachment.
    pub fn new() -> LinkNicToUpdate {
        LinkNicToUpdate {
            delete_on_vm_deletion: None,
            link_nic_id: None,
        }
    }
}
