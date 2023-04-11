/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVmType : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVmType {
    /// This parameter is not available. It is present in our API for the sake of historical compatibility with AWS.
    #[serde(rename = "BsuOptimized", skip_serializing_if = "Option::is_none")]
    pub bsu_optimized: Option<bool>,
    /// The amounts of memory, in gibibytes (GiB).
    #[serde(rename = "MemorySizes", skip_serializing_if = "Option::is_none")]
    pub memory_sizes: Option<Vec<f32>>,
    /// The numbers of vCores.
    #[serde(rename = "VcoreCounts", skip_serializing_if = "Option::is_none")]
    pub vcore_counts: Option<Vec<i32>>,
    /// The names of the VM types. For more information, see [Instance Types](https://docs.outscale.com/en/userguide/Instance-Types.html).
    #[serde(rename = "VmTypeNames", skip_serializing_if = "Option::is_none")]
    pub vm_type_names: Option<Vec<String>>,
    /// The maximum number of ephemeral storage disks.
    #[serde(rename = "VolumeCounts", skip_serializing_if = "Option::is_none")]
    pub volume_counts: Option<Vec<i32>>,
    /// The size of one ephemeral storage disk, in gibibytes (GiB).
    #[serde(rename = "VolumeSizes", skip_serializing_if = "Option::is_none")]
    pub volume_sizes: Option<Vec<i32>>,
}

impl FiltersVmType {
    /// One or more filters.
    pub fn new() -> FiltersVmType {
        FiltersVmType {
            bsu_optimized: None,
            memory_sizes: None,
            vcore_counts: None,
            vm_type_names: None,
            volume_counts: None,
            volume_sizes: None,
        }
    }
}
