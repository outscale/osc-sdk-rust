# CreateSnapshotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description for the snapshot. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**file_location** | Option<**String**> | The pre-signed URL of the snapshot you want to import from the bucket. | [optional]
**snapshot_size** | Option<**i64**> | The size of the snapshot you want to create in your account, in bytes. This size must be greater than or equal to the size of the original, uncompressed snapshot. | [optional]
**source_region_name** | Option<**String**> | The name of the source Region, which must be the same as the Region of your account. | [optional]
**source_snapshot_id** | Option<**String**> | The ID of the snapshot you want to copy. | [optional]
**volume_id** | Option<**String**> | The ID of the volume you want to create a snapshot of. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


