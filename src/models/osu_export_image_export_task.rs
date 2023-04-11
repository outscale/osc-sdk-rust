/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// OsuExportImageExportTask : Information about the OMI export task.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct OsuExportImageExportTask {
    /// The format of the export disk (`qcow2` \\| `raw`).
    #[serde(rename = "DiskImageFormat")]
    pub disk_image_format: String,
    /// The name of the OOS bucket the OMI is exported to.
    #[serde(rename = "OsuBucket")]
    pub osu_bucket: String,
    /// The URL of the manifest file.
    #[serde(rename = "OsuManifestUrl", skip_serializing_if = "Option::is_none")]
    pub osu_manifest_url: Option<String>,
    /// The prefix for the key of the OOS object corresponding to the image.
    #[serde(rename = "OsuPrefix", skip_serializing_if = "Option::is_none")]
    pub osu_prefix: Option<String>,
}

impl OsuExportImageExportTask {
    /// Information about the OMI export task.
    pub fn new(disk_image_format: String, osu_bucket: String) -> OsuExportImageExportTask {
        OsuExportImageExportTask {
            disk_image_format,
            osu_bucket,
            osu_manifest_url: None,
            osu_prefix: None,
        }
    }
}
