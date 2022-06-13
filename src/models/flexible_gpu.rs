/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FlexibleGpu : Information about the flexible GPU (fGPU).

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexibleGpu {
    /// If true, the fGPU is deleted when the VM is terminated.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The ID of the fGPU.
    #[serde(rename = "FlexibleGpuId", skip_serializing_if = "Option::is_none")]
    pub flexible_gpu_id: Option<String>,
    /// The compatible processor generation.
    #[serde(rename = "Generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    /// The model of fGPU. For more information, see [About Flexible GPUs](https://docs.outscale.com/en/userguide/About-Flexible-GPUs.html).
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The state of the fGPU (`allocated` \\| `attaching` \\| `attached` \\| `detaching`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The Subregion where the fGPU is located.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// The ID of the VM the fGPU is attached to, if any.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl FlexibleGpu {
    /// Information about the flexible GPU (fGPU).
    pub fn new() -> FlexibleGpu {
        FlexibleGpu {
            delete_on_vm_deletion: None,
            flexible_gpu_id: None,
            generation: None,
            model_name: None,
            state: None,
            subregion_name: None,
            vm_id: None,
        }
    }
}
