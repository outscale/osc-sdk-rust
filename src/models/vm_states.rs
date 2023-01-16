/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VmStates : Information about the states of the VMs.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VmStates {
    /// One or more scheduled events associated with the VM.
    #[serde(rename = "MaintenanceEvents", skip_serializing_if = "Option::is_none")]
    pub maintenance_events: Option<Vec<crate::models::MaintenanceEvent>>,
    /// The name of the Subregion of the VM.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// The ID of the VM.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// The state of the VM (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`).
    #[serde(rename = "VmState", skip_serializing_if = "Option::is_none")]
    pub vm_state: Option<String>,
}

impl VmStates {
    /// Information about the states of the VMs.
    pub fn new() -> VmStates {
        VmStates {
            maintenance_events: None,
            subregion_name: None,
            vm_id: None,
            vm_state: None,
        }
    }
}
