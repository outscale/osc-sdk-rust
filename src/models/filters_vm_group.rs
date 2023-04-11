/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVmGroup : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVmGroup {
    /// The descriptions of the VM groups.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The IDs of the security groups.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The IDs of the Subnets.
    #[serde(rename = "SubnetIds", skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// The keys of the tags associated with the VM groups.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the VM groups.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the VMs, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The number of VMs in the VM group.
    #[serde(rename = "VmCounts", skip_serializing_if = "Option::is_none")]
    pub vm_counts: Option<Vec<i32>>,
    /// The IDs of the VM groups.
    #[serde(rename = "VmGroupIds", skip_serializing_if = "Option::is_none")]
    pub vm_group_ids: Option<Vec<String>>,
    /// The names of the VM groups.
    #[serde(rename = "VmGroupNames", skip_serializing_if = "Option::is_none")]
    pub vm_group_names: Option<Vec<String>>,
    /// The IDs of the VM templates.
    #[serde(rename = "VmTemplateIds", skip_serializing_if = "Option::is_none")]
    pub vm_template_ids: Option<Vec<String>>,
}

impl FiltersVmGroup {
    /// One or more filters.
    pub fn new() -> FiltersVmGroup {
        FiltersVmGroup {
            descriptions: None,
            security_group_ids: None,
            subnet_ids: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            vm_counts: None,
            vm_group_ids: None,
            vm_group_names: None,
            vm_template_ids: None,
        }
    }
}
