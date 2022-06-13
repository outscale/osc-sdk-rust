/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// BsuToUpdateVm : Information about the BSU volume.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BsuToUpdateVm {
    /// If set to true, the volume is deleted when terminating the VM. If set to false, the volume is not deleted when terminating the VM.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The ID of the volume.
    #[serde(rename = "VolumeId", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

impl BsuToUpdateVm {
    /// Information about the BSU volume.
    pub fn new() -> BsuToUpdateVm {
        BsuToUpdateVm {
            delete_on_vm_deletion: None,
            volume_id: None,
        }
    }
}
