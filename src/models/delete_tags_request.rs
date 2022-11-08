/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteTagsRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// One or more resource IDs.
    #[serde(rename = "ResourceIds")]
    pub resource_ids: Vec<String>,
    /// One or more tags to delete (if you set a tag value, only the tags matching exactly this value are deleted).
    #[serde(rename = "Tags")]
    pub tags: Vec<crate::models::ResourceTag>,
}

impl DeleteTagsRequest {
    pub fn new(
        resource_ids: Vec<String>,
        tags: Vec<crate::models::ResourceTag>,
    ) -> DeleteTagsRequest {
        DeleteTagsRequest {
            dry_run: None,
            resource_ids,
            tags,
        }
    }
}
