/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// OsuExportSnapshotExportTask : Information about the snapshot export task.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OsuExportSnapshotExportTask {
    /// The format of the export disk (`qcow2` \\| `raw`).
    #[serde(rename = "DiskImageFormat")]
    pub disk_image_format: String,
    /// The name of the OOS bucket the snapshot is exported to.
    #[serde(rename = "OsuBucket")]
    pub osu_bucket: String,
    /// The prefix for the key of the OOS object corresponding to the snapshot.
    #[serde(rename = "OsuPrefix", skip_serializing_if = "Option::is_none")]
    pub osu_prefix: Option<String>,
}

impl OsuExportSnapshotExportTask {
    /// Information about the snapshot export task.
    pub fn new(disk_image_format: String, osu_bucket: String) -> OsuExportSnapshotExportTask {
        OsuExportSnapshotExportTask {
            disk_image_format,
            osu_bucket,
            osu_prefix: None,
        }
    }
}
