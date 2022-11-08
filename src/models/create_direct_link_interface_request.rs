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
pub struct CreateDirectLinkInterfaceRequest {
    /// The ID of the existing DirectLink for which you want to create the DirectLink interface.
    #[serde(rename = "DirectLinkId")]
    pub direct_link_id: String,
    #[serde(rename = "DirectLinkInterface")]
    pub direct_link_interface: Box<crate::models::DirectLinkInterface>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl CreateDirectLinkInterfaceRequest {
    pub fn new(
        direct_link_id: String,
        direct_link_interface: crate::models::DirectLinkInterface,
    ) -> CreateDirectLinkInterfaceRequest {
        CreateDirectLinkInterfaceRequest {
            direct_link_id,
            direct_link_interface: Box::new(direct_link_interface),
            dry_run: None,
        }
    }
}
