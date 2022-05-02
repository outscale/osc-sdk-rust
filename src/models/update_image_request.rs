/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateImageRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The ID of the OMI you want to modify.
    #[serde(rename = "ImageId")]
    pub image_id: String,
    #[serde(rename = "PermissionsToLaunch")]
    pub permissions_to_launch: Box<crate::models::PermissionsOnResourceCreation>,
}

impl UpdateImageRequest {
    pub fn new(
        image_id: String,
        permissions_to_launch: crate::models::PermissionsOnResourceCreation,
    ) -> UpdateImageRequest {
        UpdateImageRequest {
            dry_run: None,
            image_id,
            permissions_to_launch: Box::new(permissions_to_launch),
        }
    }
}
