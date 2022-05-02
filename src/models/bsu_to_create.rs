/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// BsuToCreate : Information about the BSU volume to create.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BsuToCreate {
    /// By default or if set to true, the volume is deleted when terminating the VM. If false, the volume is not deleted when terminating the VM.
    #[serde(rename = "DeleteOnVmDeletion", skip_serializing_if = "Option::is_none")]
    pub delete_on_vm_deletion: Option<bool>,
    /// The number of I/O operations per second (IOPS). This parameter must be specified only if you create an `io1` volume. The maximum number of IOPS allowed for `io1` volumes is `13000`.
    #[serde(rename = "Iops", skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    /// The ID of the snapshot used to create the volume.
    #[serde(rename = "SnapshotId", skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// The size of the volume, in gibibytes (GiB).<br /> If you specify a snapshot ID, the volume size must be at least equal to the snapshot size.<br /> If you specify a snapshot ID but no volume size, the volume is created with a size similar to the snapshot one.
    #[serde(rename = "VolumeSize", skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    /// The type of the volume (`standard` \\| `io1` \\| `gp2`). If not specified in the request, a `standard` volume is created.<br /> For more information about volume types, see [About Volumes > Volume Types and IOPS](https://docs.outscale.com/en/userguide/About-Volumes.html#_volume_types_and_iops).
    #[serde(rename = "VolumeType", skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

impl BsuToCreate {
    /// Information about the BSU volume to create.
    pub fn new() -> BsuToCreate {
        BsuToCreate {
            delete_on_vm_deletion: None,
            iops: None,
            snapshot_id: None,
            volume_size: None,
            volume_type: None,
        }
    }
}
