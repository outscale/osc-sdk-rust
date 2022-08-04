/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersSnapshot : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersSnapshot {
    /// The account aliases of the owners of the snapshots.
    #[serde(rename = "AccountAliases", skip_serializing_if = "Option::is_none")]
    pub account_aliases: Option<Vec<String>>,
    /// The account IDs of the owners of the snapshots.
    #[serde(rename = "AccountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// The descriptions of the snapshots.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The account IDs of one or more users who have permissions to create volumes.
    #[serde(
        rename = "PermissionsToCreateVolumeAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_to_create_volume_account_ids: Option<Vec<String>>,
    /// If true, lists all public volumes. If false, lists all private volumes.
    #[serde(
        rename = "PermissionsToCreateVolumeGlobalPermission",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_to_create_volume_global_permission: Option<bool>,
    /// The progresses of the snapshots, as a percentage.
    #[serde(rename = "Progresses", skip_serializing_if = "Option::is_none")]
    pub progresses: Option<Vec<i32>>,
    /// The IDs of the snapshots.
    #[serde(rename = "SnapshotIds", skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<String>>,
    /// The states of the snapshots (`in-queue` \\| `completed` \\| `error`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the snapshots.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the snapshots.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the snapshots, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The IDs of the volumes used to create the snapshots.
    #[serde(rename = "VolumeIds", skip_serializing_if = "Option::is_none")]
    pub volume_ids: Option<Vec<String>>,
    /// The sizes of the volumes used to create the snapshots, in gibibytes (GiB).
    #[serde(rename = "VolumeSizes", skip_serializing_if = "Option::is_none")]
    pub volume_sizes: Option<Vec<i32>>,
}

impl FiltersSnapshot {
    /// One or more filters.
    pub fn new() -> FiltersSnapshot {
        FiltersSnapshot {
            account_aliases: None,
            account_ids: None,
            descriptions: None,
            permissions_to_create_volume_account_ids: None,
            permissions_to_create_volume_global_permission: None,
            progresses: None,
            snapshot_ids: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            volume_ids: None,
            volume_sizes: None,
        }
    }
}
