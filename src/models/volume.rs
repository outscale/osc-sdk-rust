/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Volume : Information about the volume.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Volume {
    /// The date and time of creation of the volume.
    #[serde(rename = "CreationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The number of I/O operations per second (IOPS):<br /> - For `io1` volumes, the number of provisioned IOPS<br /> - For `gp2` volumes, the baseline performance of the volume
    #[serde(rename = "Iops", skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    /// Information about your volume attachment.
    #[serde(rename = "LinkedVolumes", skip_serializing_if = "Option::is_none")]
    pub linked_volumes: Option<Vec<crate::models::LinkedVolume>>,
    /// The size of the volume, in gibibytes (GiB).
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The snapshot from which the volume was created.
    #[serde(rename = "SnapshotId", skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// The state of the volume (`creating` \\| `available` \\| `in-use` \\| `updating` \\| `deleting` \\| `error`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The Subregion in which the volume was created.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// One or more tags associated with the volume.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// The ID of the volume.
    #[serde(rename = "VolumeId", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// The type of the volume (`standard` \\| `gp2` \\| `io1`).
    #[serde(rename = "VolumeType", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

impl Volume {
    /// Information about the volume.
    pub fn new() -> Volume {
        Volume {
            creation_date: None,
            iops: None,
            linked_volumes: None,
            size: None,
            snapshot_id: None,
            state: None,
            subregion_name: None,
            tags: None,
            volume_id: None,
            volume_type: None,
        }
    }
}
