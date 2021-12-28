/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersImage : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersImage {
    /// The account aliases of the owners of the OMIs.
    #[serde(rename = "AccountAliases", skip_serializing_if = "Option::is_none")]
    pub account_aliases: Option<Vec<String>>,
    /// The account IDs of the owners of the OMIs. By default, all the OMIs for which you have launch permissions are described.
    #[serde(rename = "AccountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// The architectures of the OMIs (`i386` \\| `x86_64`).
    #[serde(rename = "Architectures", skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<String>>,
    /// Whether the volumes are deleted or not when terminating the VM.
    #[serde(
        rename = "BlockDeviceMappingDeleteOnVmDeletion",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_delete_on_vm_deletion: Option<bool>,
    /// The device names for the volumes.
    #[serde(
        rename = "BlockDeviceMappingDeviceNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_device_names: Option<Vec<String>>,
    /// The IDs of the snapshots used to create the volumes.
    #[serde(
        rename = "BlockDeviceMappingSnapshotIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_snapshot_ids: Option<Vec<String>>,
    /// The sizes of the volumes, in gibibytes (GiB).
    #[serde(
        rename = "BlockDeviceMappingVolumeSizes",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_volume_sizes: Option<Vec<i32>>,
    /// The types of volumes (`standard` \\| `gp2` \\| `io1`).
    #[serde(
        rename = "BlockDeviceMappingVolumeTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub block_device_mapping_volume_types: Option<Vec<String>>,
    /// The descriptions of the OMIs, provided when they were created.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The locations of the buckets where the OMI files are stored.
    #[serde(rename = "FileLocations", skip_serializing_if = "Option::is_none")]
    pub file_locations: Option<Vec<String>>,
    /// The hypervisor type of the OMI (always `xen`).
    #[serde(rename = "Hypervisors", skip_serializing_if = "Option::is_none")]
    pub hypervisors: Option<Vec<String>>,
    /// The IDs of the OMIs.
    #[serde(rename = "ImageIds", skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<String>>,
    /// The names of the OMIs, provided when they were created.
    #[serde(rename = "ImageNames", skip_serializing_if = "Option::is_none")]
    pub image_names: Option<Vec<String>>,
    /// The account IDs of the users who have launch permissions for the OMIs.
    #[serde(
        rename = "PermissionsToLaunchAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_to_launch_account_ids: Option<Vec<String>>,
    /// If true, lists all public OMIs. If false, lists all private OMIs.
    #[serde(
        rename = "PermissionsToLaunchGlobalPermission",
        skip_serializing_if = "Option::is_none"
    )]
    pub permissions_to_launch_global_permission: Option<bool>,
    /// The product code associated with the OMI (`0001` Linux/Unix \\| `0002` Windows \\| `0004` Linux/Oracle \\| `0005` Windows 10).
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// The device names of the root devices (for example, `/dev/sda1`).
    #[serde(rename = "RootDeviceNames", skip_serializing_if = "Option::is_none")]
    pub root_device_names: Option<Vec<String>>,
    /// The types of root device used by the OMIs (always `bsu`).
    #[serde(rename = "RootDeviceTypes", skip_serializing_if = "Option::is_none")]
    pub root_device_types: Option<Vec<String>>,
    /// The states of the OMIs (`pending` \\| `available` \\| `failed`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the OMIs.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the OMIs.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the OMIs, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The virtualization types (always `hvm`).
    #[serde(
        rename = "VirtualizationTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtualization_types: Option<Vec<String>>,
}

impl FiltersImage {
    /// One or more filters.
    pub fn new() -> FiltersImage {
        FiltersImage {
            account_aliases: None,
            account_ids: None,
            architectures: None,
            block_device_mapping_delete_on_vm_deletion: None,
            block_device_mapping_device_names: None,
            block_device_mapping_snapshot_ids: None,
            block_device_mapping_volume_sizes: None,
            block_device_mapping_volume_types: None,
            descriptions: None,
            file_locations: None,
            hypervisors: None,
            image_ids: None,
            image_names: None,
            permissions_to_launch_account_ids: None,
            permissions_to_launch_global_permission: None,
            product_codes: None,
            root_device_names: None,
            root_device_types: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            virtualization_types: None,
        }
    }
}
