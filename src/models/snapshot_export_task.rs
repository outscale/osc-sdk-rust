/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SnapshotExportTask : Information about the snapshot export task.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotExportTask {
    /// If the snapshot export task fails, an error message appears.
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "OsuExport", skip_serializing_if = "Option::is_none")]
    pub osu_export: Option<Box<crate::models::OsuExportSnapshotExportTask>>,
    /// The progress of the snapshot export task, as a percentage.
    #[serde(rename = "Progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// The ID of the snapshot to be exported.
    #[serde(rename = "SnapshotId", skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// The state of the snapshot export task (`pending` \\| `active` \\| `completed` \\| `failed`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the snapshot export task.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The ID of the snapshot export task.
    #[serde(rename = "TaskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl SnapshotExportTask {
    /// Information about the snapshot export task.
    pub fn new() -> SnapshotExportTask {
        SnapshotExportTask {
            comment: None,
            osu_export: None,
            progress: None,
            snapshot_id: None,
            state: None,
            tags: None,
            task_id: None,
        }
    }
}
