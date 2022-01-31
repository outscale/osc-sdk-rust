/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FlexibleGpuCatalog : Information about the flexible GPU (fGPU) that is available in the public catalog.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexibleGpuCatalog {
    /// The generations of VMs that the fGPU is compatible with.
    #[serde(rename = "Generations", skip_serializing_if = "Option::is_none")]
    pub generations: Option<Vec<String>>,
    /// The maximum number of VM vCores that the fGPU is compatible with.
    #[serde(rename = "MaxCpu", skip_serializing_if = "Option::is_none")]
    pub max_cpu: Option<i32>,
    /// The maximum amount of VM memory that the fGPU is compatible with.
    #[serde(rename = "MaxRam", skip_serializing_if = "Option::is_none")]
    pub max_ram: Option<i32>,
    /// The model of fGPU.
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The amount of video RAM (VRAM) of the fGPU.
    #[serde(rename = "VRam", skip_serializing_if = "Option::is_none")]
    pub v_ram: Option<i32>,
}

impl FlexibleGpuCatalog {
    /// Information about the flexible GPU (fGPU) that is available in the public catalog.
    pub fn new() -> FlexibleGpuCatalog {
        FlexibleGpuCatalog {
            generations: None,
            max_cpu: None,
            max_ram: None,
            model_name: None,
            v_ram: None,
        }
    }
}
