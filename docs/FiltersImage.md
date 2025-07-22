# FiltersImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_aliases** | Option<**Vec<String>**> | The account aliases of the owners of the OMIs. | [optional]
**account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the OMIs. By default, all the OMIs for which you have launch permissions are described. | [optional]
**architectures** | Option<**Vec<String>**> | The architectures of the OMIs (`i386` \\| `x86_64`). | [optional]
**block_device_mapping_delete_on_vm_deletion** | Option<**bool**> | Whether the volumes are deleted or not when terminating the VM. | [optional]
**block_device_mapping_device_names** | Option<**Vec<String>**> | The device names for the volumes. | [optional]
**block_device_mapping_snapshot_ids** | Option<**Vec<String>**> | The IDs of the snapshots used to create the volumes. | [optional]
**block_device_mapping_volume_sizes** | Option<**Vec<i32>**> | The sizes of the volumes, in gibibytes (GiB). | [optional]
**block_device_mapping_volume_types** | Option<**Vec<String>**> | The types of volumes (`standard` \\| `gp2` \\| `io1`). | [optional]
**boot_modes** | Option<[**Vec<crate::models::BootMode>**](BootMode.md)> | The boot modes compatible with the OMIs. | [optional]
**descriptions** | Option<**Vec<String>**> | The descriptions of the OMIs, provided when they were created. | [optional]
**file_locations** | Option<**Vec<String>**> | The locations of the buckets where the OMI files are stored. | [optional]
**hypervisors** | Option<**Vec<String>**> | The hypervisor type of the OMI (always `xen`). | [optional]
**image_ids** | Option<**Vec<String>**> | The IDs of the OMIs. | [optional]
**image_names** | Option<**Vec<String>**> | The names of the OMIs, provided when they were created. | [optional]
**permissions_to_launch_account_ids** | Option<**Vec<String>**> | The account IDs which have launch permissions for the OMIs. | [optional]
**permissions_to_launch_global_permission** | Option<**bool**> | If true, lists all public OMIs. If false, lists all private OMIs. | [optional]
**product_code_names** | Option<**Vec<String>**> | The names of the product codes associated with the OMI. | [optional]
**product_codes** | Option<**Vec<String>**> | The product codes associated with the OMI. | [optional]
**root_device_names** | Option<**Vec<String>**> | The name of the root device. This value must be /dev/sda1. | [optional]
**root_device_types** | Option<**Vec<String>**> | The types of root device used by the OMIs (`bsu` or `ebs`). | [optional]
**secure_boot** | Option<**bool**> | Whether secure boot is activated or not. | [optional]
**states** | Option<**Vec<String>**> | The states of the OMIs (`pending` \\| `available` \\| `failed`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the OMIs. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the OMIs. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the OMIs, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]
**virtualization_types** | Option<**Vec<String>**> | The virtualization types (always `hvm`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


