/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// The architecture of the OMI (by default, `i386` if you specified the `FileLocation` or `RootDeviceName` parameter).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// One or more block device mappings.
    #[serde(
        rename = "BlockDeviceMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mappings: Option<Vec<crate::models::BlockDeviceMappingImage>>,
    /// A description for the new OMI.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The pre-signed URL of the OMI manifest file, or the full path to the OMI stored in a bucket. If you specify this parameter, a copy of the OMI is created in your account. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`.
    #[serde(rename = "FileLocation", skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    /// A unique name for the new OMI.<br /> Constraints: 3-128 alphanumeric characters, underscores (_), spaces ( ), parentheses (()), slashes (/), periods (.), or dashes (-).
    #[serde(rename = "ImageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// If false, the VM shuts down before creating the OMI and then reboots. If true, the VM does not.
    #[serde(rename = "NoReboot", skip_serializing_if = "Option::is_none")]
    pub no_reboot: Option<bool>,
    /// The name of the root device. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`.
    #[serde(rename = "RootDeviceName", skip_serializing_if = "Option::is_none")]
    pub root_device_name: Option<String>,
    /// The ID of the OMI you want to copy. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`.
    #[serde(rename = "SourceImageId", skip_serializing_if = "Option::is_none")]
    pub source_image_id: Option<String>,
    /// The name of the source Region, which must be the same as the Region of your account.
    #[serde(rename = "SourceRegionName", skip_serializing_if = "Option::is_none")]
    pub source_region_name: Option<String>,
    /// The ID of the VM from which you want to create the OMI. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`.
    #[serde(rename = "VmId", skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
}

impl CreateImageRequest {
    pub fn new() -> CreateImageRequest {
        CreateImageRequest {
            architecture: None,
            block_device_mappings: None,
            description: None,
            dry_run: None,
            file_location: None,
            image_name: None,
            no_reboot: None,
            root_device_name: None,
            source_image_id: None,
            source_region_name: None,
            vm_id: None,
        }
    }
}
