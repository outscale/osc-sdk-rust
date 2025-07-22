# FiltersVolume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_tokens** | Option<**Vec<String>**> | The idempotency tokens provided when creating the volumes. | [optional]
**creation_dates** | Option<**Vec<String>**> | The dates and times at which the volumes were created, in ISO 8601 date-time format (for example, `2020-06-30T00:00:00.000Z`). | [optional]
**link_volume_delete_on_vm_deletion** | Option<**bool**> | Whether the volumes are deleted or not when terminating the VMs. | [optional]
**link_volume_device_names** | Option<**Vec<String>**> | The VM device names. | [optional]
**link_volume_link_dates** | Option<**Vec<String>**> | The dates and times at which the volumes were attached, in ISO 8601 date-time format (for example, `2020-06-30T00:00:00.000Z`). | [optional]
**link_volume_link_states** | Option<**Vec<String>**> | The attachment states of the volumes (`attaching` \\| `detaching` \\| `attached` \\| `detached`). | [optional]
**link_volume_vm_ids** | Option<**Vec<String>**> | One or more IDs of VMs. | [optional]
**snapshot_ids** | Option<**Vec<String>**> | The snapshots from which the volumes were created. | [optional]
**subregion_names** | Option<**Vec<String>**> | The names of the Subregions in which the volumes were created. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the volumes. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the volumes. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the volumes, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]
**volume_ids** | Option<**Vec<String>**> | The IDs of the volumes. | [optional]
**volume_sizes** | Option<**Vec<i32>**> | The sizes of the volumes, in gibibytes (GiB). | [optional]
**volume_states** | Option<**Vec<String>**> | The states of the volumes (`creating` \\| `available` \\| `in-use` \\| `deleting` \\| `error`). | [optional]
**volume_types** | Option<**Vec<String>**> | The types of the volumes (`standard` \\| `gp2` \\| `io1`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


