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
pub struct CreateSnapshotRequest {
    /// A description for the snapshot.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The pre-signed URL of the snapshot you want to import from the bucket.
    #[serde(rename = "FileLocation", skip_serializing_if = "Option::is_none")]
    pub file_location: Option<String>,
    /// The size of the snapshot you want to create in your account, in bytes. This size must be greater than or equal to the size of the original, uncompressed snapshot.
    #[serde(rename = "SnapshotSize", skip_serializing_if = "Option::is_none")]
    pub snapshot_size: Option<i64>,
    /// The name of the source Region, which must be the same as the Region of your account.
    #[serde(rename = "SourceRegionName", skip_serializing_if = "Option::is_none")]
    pub source_region_name: Option<String>,
    /// The ID of the snapshot you want to copy.
    #[serde(rename = "SourceSnapshotId", skip_serializing_if = "Option::is_none")]
    pub source_snapshot_id: Option<String>,
    /// The ID of the volume you want to create a snapshot of.
    #[serde(rename = "VolumeId", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

impl CreateSnapshotRequest {
    pub fn new() -> CreateSnapshotRequest {
        CreateSnapshotRequest {
            description: None,
            dry_run: None,
            file_location: None,
            snapshot_size: None,
            source_region_name: None,
            source_snapshot_id: None,
            volume_id: None,
        }
    }
}
