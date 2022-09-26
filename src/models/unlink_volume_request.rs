/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UnlinkVolumeRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// Forces the detachment of the volume in case of previous failure. Important: This action may damage your data or file systems.
    #[serde(rename = "ForceUnlink", skip_serializing_if = "Option::is_none")]
    pub force_unlink: Option<bool>,
    /// The ID of the volume you want to detach.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
}

impl UnlinkVolumeRequest {
    pub fn new(volume_id: String) -> UnlinkVolumeRequest {
        UnlinkVolumeRequest {
            dry_run: None,
            force_unlink: None,
            volume_id,
        }
    }
}
