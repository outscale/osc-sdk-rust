/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Image : Information about the OMI.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    /// The account alias of the owner of the OMI.
    #[serde(rename = "AccountAlias", skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<String>,
    /// The account ID of the owner of the OMI.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The architecture of the OMI (by default, `i386`).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// One or more block device mappings.
    #[serde(
        rename = "BlockDeviceMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mappings: Option<Vec<crate::models::BlockDeviceMappingImage>>,
    /// The date and time at which the OMI was created.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The description of the OMI.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The location of the bucket where the OMI files are stored.
    #[serde(rename = "FileLocation", skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    /// The ID of the OMI.
    #[serde(rename = "ImageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// The name of the OMI.
    #[serde(rename = "ImageName", skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    /// The type of the OMI.
    #[serde(rename = "ImageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<String>,
    #[serde(
        rename = "PermissionsToLaunch",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_to_launch: Option<Box<crate::models::PermissionsOnResource>>,
    /// The product code associated with the OMI (`0001` Linux/Unix \\| `0002` Windows \\| `0004` Linux/Oracle \\| `0005` Windows 10).
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// The name of the root device.
    #[serde(rename = "RootDeviceName", skip_serializing_if = "Option::is_none")]
    pub root_device_name: Option<String>,
    /// The type of root device used by the OMI (always `bsu`).
    #[serde(rename = "RootDeviceType", skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    /// The state of the OMI (`pending` \\| `available` \\| `failed`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateComment", skip_serializing_if = "Option::is_none")]
    pub state_comment: Option<Box<crate::models::StateComment>>,
    /// One or more tags associated with the OMI.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl Image {
    /// Information about the OMI.
    pub fn new() -> Image {
        Image {
            account_alias: None,
            account_id: None,
            architecture: None,
            block_device_mappings: None,
            creation_date: None,
            description: None,
            file_location: None,
            image_id: None,
            image_name: None,
            image_type: None,
            permissions_to_launch: None,
            product_codes: None,
            root_device_name: None,
            root_device_type: None,
            state: None,
            state_comment: None,
            tags: None,
        }
    }
}
