/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.17
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSnapshotExportTaskRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "OsuExport")]
    pub osu_export: Box<crate::models::OsuExportToCreate>,
    /// The ID of the snapshot to export.
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

impl CreateSnapshotExportTaskRequest {
    pub fn new(
        osu_export: crate::models::OsuExportToCreate,
        snapshot_id: String,
    ) -> CreateSnapshotExportTaskRequest {
        CreateSnapshotExportTaskRequest {
            dry_run: None,
            osu_export: Box::new(osu_export),
            snapshot_id,
        }
    }
}
