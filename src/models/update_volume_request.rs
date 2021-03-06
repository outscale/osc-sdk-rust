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
pub struct UpdateVolumeRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The new number of I/O operations per second (IOPS). This parameter can be specified only if you update an `io1` volume. The maximum number of IOPS allowed for `io1` volumes is `13000`. This modification is instantaneous on a cold volume, not on a hot one.
    #[serde(rename = "Iops", skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    /// (cold volume only) The new size of the volume, in gibibytes (GiB). This value must be equal to or greater than the current size of the volume. This modification is not instantaneous.
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The ID of the volume you want to update.
    #[serde(rename = "VolumeId")]
    pub volume_id: String,
    /// (cold volume only) The new type of the volume (`standard` \\| `io1` \\| `gp2`). This modification is instantaneous. If you update to an `io1` volume, you must also specify the `Iops` parameter.
    #[serde(rename = "VolumeType", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

impl UpdateVolumeRequest {
    pub fn new(volume_id: String) -> UpdateVolumeRequest {
        UpdateVolumeRequest {
            dry_run: None,
            iops: None,
            size: None,
            volume_id,
            volume_type: None,
        }
    }
}
