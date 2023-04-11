/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVmTemplate : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVmTemplate {
    /// The number of vCores.
    #[serde(rename = "CpuCores", skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<Vec<i32>>,
    /// The processor generations (for example, `v4`).
    #[serde(rename = "CpuGenerations", skip_serializing_if = "Option::is_none")]
    pub cpu_generations: Option<Vec<String>>,
    /// The performances of the VMs.
    #[serde(rename = "CpuPerformances", skip_serializing_if = "Option::is_none")]
    pub cpu_performances: Option<Vec<String>>,
    /// The descriptions of the VM templates.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The IDs of the OMIs.
    #[serde(rename = "ImageIds", skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    /// The names of the keypairs.
    #[serde(rename = "KeypairNames", skip_serializing_if = "Option::is_none")]
    pub keypair_names: Option<Vec<String>>,
    /// The amount of RAM.
    #[serde(rename = "Rams", skip_serializing_if = "Option::is_none")]
    pub rams: Option<Vec<i32>>,
    /// The keys of the tags associated with the VM templates.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the VM templates.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the VM templates, in the following format: \"Filters\":{\"Tags\":[\"TAGKEY=TAGVALUE\"]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The IDs of the VM templates.
    #[serde(rename = "VmTemplateIds", skip_serializing_if = "Option::is_none")]
    pub vm_template_ids: Option<Vec<String>>,
    /// The names of the VM templates.
    #[serde(rename = "VmTemplateNames", skip_serializing_if = "Option::is_none")]
    pub vm_template_names: Option<Vec<String>>,
}

impl FiltersVmTemplate {
    /// One or more filters.
    pub fn new() -> FiltersVmTemplate {
        FiltersVmTemplate {
            cpu_cores: None,
            cpu_generations: None,
            cpu_performances: None,
            descriptions: None,
            image_ids: None,
            keypair_names: None,
            rams: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            vm_template_ids: None,
            vm_template_names: None,
        }
    }
}