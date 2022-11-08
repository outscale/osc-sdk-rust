/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// LinkedVolume : Information about volume attachment.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LinkedVolume {
    /// If true, the volume is deleted when terminating the VM. If false, the volume is not deleted when terminating the VM.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The name of the device.
    #[serde(rename = "DeviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// The state of the attachment of the volume (`attaching` \\| `detaching` \\| `attached` \\| `detached`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// The ID of the volume.
    #[serde(rename = "VolumeId", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

impl LinkedVolume {
    /// Information about volume attachment.
    pub fn new() -> LinkedVolume {
        LinkedVolume {
            delete_on_vm_deletion: None,
            device_name: None,
            state: None,
            vm_id: None,
            volume_id: None,
        }
    }
}
