/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ImageExportTask : Information about the OMI export task.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageExportTask {
    /// If the OMI export task fails, an error message appears.
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The ID of the OMI to be exported.
    #[serde(rename = "ImageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "OsuExport", skip_serializing_if = "Option::is_none")]
    pub osu_export: Option<Box<crate::models::OsuExportImageExportTask>>,
    /// The progress of the OMI export task, as a percentage.
    #[serde(rename = "Progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// The state of the OMI export task (`pending/queued` \\| `pending` \\| `completed` \\| `failed` \\| `cancelled`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// One or more tags associated with the image export task.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The ID of the OMI export task.
    #[serde(rename = "TaskId", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ImageExportTask {
    /// Information about the OMI export task.
    pub fn new() -> ImageExportTask {
        ImageExportTask {
            comment: None,
            image_id: None,
            osu_export: None,
            progress: None,
            state: None,
            tags: None,
            task_id: None,
        }
    }
}
