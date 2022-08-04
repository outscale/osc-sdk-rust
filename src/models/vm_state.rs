/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VmState : Information about the state of the VM.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VmState {
    /// The current state of the VM (`InService` \\| `OutOfService` \\| `Unknown`).
    #[serde(rename = "CurrentState", skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    /// The previous state of the VM (`InService` \\| `OutOfService` \\| `Unknown`).
    #[serde(rename = "PreviousState", skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl VmState {
    /// Information about the state of the VM.
    pub fn new() -> VmState {
        VmState {
            current_state: None,
            previous_state: None,
            vm_id: None,
        }
    }
}
