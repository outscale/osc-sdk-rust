# Snapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_alias** | Option<**String**> | The account alias of the owner of the snapshot. | [optional]
**account_id** | Option<**String**> | The account ID of the owner of the snapshot. | [optional]
**creation_date** | Option<**String**> | The date and time (UTC) at which the snapshot was created. | [optional]
**description** | Option<**String**> | The description of the snapshot. | [optional]
**permissions_to_create_volume** | Option<[**crate::models::PermissionsOnResource**](PermissionsOnResource.md)> |  | [optional]
**progress** | Option<**i32**> | The progress of the snapshot, as a percentage. | [optional]
**snapshot_id** | Option<**String**> | The ID of the snapshot. | [optional]
**state** | Option<**String**> | The state of the snapshot (`in-queue` \\| `pending` \\| `completed` \\| `error` \\| `deleting`)). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the snapshot. | [optional]
**volume_id** | Option<**String**> | The ID of the volume used to create the snapshot. | [optional]
**volume_size** | Option<**i32**> | The size of the volume used to create the snapshot, in gibibytes (GiB). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


