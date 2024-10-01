/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html). # Pagination Tutorial You can learn more about the pagination methods for read calls in the dedicated [pagination tutorial](https://docs.outscale.com/en/userguide/Tutorial-Paginating-an-API-Request.html). # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.33.1
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
    /// The account IDs which have launch permissions for the OMIs.
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
    /// The names of the product codes associated with the OMI.
    #[serde(rename = "ProductCodeNames", skip_serializing_if = "Option::is_none")]
    pub product_code_names: Option<Vec<String>>,
    /// The product codes associated with the OMI.
    #[serde(rename = "ProductCodes", skip_serializing_if = "Option::is_none")]
    pub product_codes: Option<Vec<String>>,
    /// The name of the root device. This value must be /dev/sda1.
    #[serde(rename = "RootDeviceNames", skip_serializing_if = "Option::is_none")]
    pub root_device_names: Option<Vec<String>>,
    /// The types of root device used by the OMIs (`bsu` or `ebs`).
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
            product_code_names: None,
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
