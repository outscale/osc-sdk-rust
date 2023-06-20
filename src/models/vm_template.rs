/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VmTemplate : Information about the VM template.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VmTemplate {
    /// The number of vCores.
    #[serde(rename = "CpuCores")]
    pub cpu_cores: i32,
    /// The processor generation.
    #[serde(rename = "CpuGeneration")]
    pub cpu_generation: String,
    /// The performance of the VMs.
    #[serde(rename = "CpuPerformance", skip_serializing_if = "Option::is_none")]
    pub cpu_performance: Option<CpuPerformance>,
    /// The date and time of creation of the VM template.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The description of the VM template.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the OMI.
    #[serde(rename = "ImageId")]
    pub image_id: String,
    /// The name of the keypair.
    #[serde(rename = "KeypairName", skip_serializing_if = "Option::is_none")]
    pub keypair_name: Option<String>,
    /// The amount of RAM.
    #[serde(rename = "Ram")]
    pub ram: i32,
    /// One or more tags associated with the VM template.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The ID of the VM template.
    #[serde(rename = "VmTemplateId")]
    pub vm_template_id: String,
    /// The name of the VM template.
    #[serde(rename = "VmTemplateName")]
    pub vm_template_name: String,
}

impl VmTemplate {
    /// Information about the VM template.
    pub fn new(
        cpu_cores: i32,
        cpu_generation: String,
        image_id: String,
        ram: i32,
        vm_template_id: String,
        vm_template_name: String,
    ) -> VmTemplate {
        VmTemplate {
            cpu_cores,
            cpu_generation,
            cpu_performance: None,
            creation_date: None,
            description: None,
            image_id,
            keypair_name: None,
            ram,
            tags: None,
            vm_template_id,
            vm_template_name,
        }
    }
}

/// The performance of the VMs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CpuPerformance {
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "highest")]
    Highest,
}

impl Default for CpuPerformance {
    fn default() -> CpuPerformance {
        Self::Medium
    }
}
