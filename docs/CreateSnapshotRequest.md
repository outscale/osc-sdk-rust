# CreateSnapshotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description for the snapshot. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**file_location** | Option<**String**> | (When importing) The pre-signed URL of the snapshot you want to import, or the normal URL of the snapshot if you have permission on the OOS bucket. For more information, see [Configuring a Pre-signed URL](https://docs.outscale.com/en/userguide/Configuring-a-Pre-signed-URL.html) or [Managing Access to Your Buckets and Objects](https://docs.outscale.com/en/userguide/Managing-Access-to-Your-Buckets-and-Objects.html). | [optional]
**snapshot_size** | Option<**i64**> | (When importing) The size of the snapshot you want to create in your account, in bytes. This size must be greater than or equal to the size of the original, uncompressed snapshot. | [optional]
**source_region_name** | Option<**String**> | (When copying) The name of the source Region, which must be the same as the Region of your account. | [optional]
**source_snapshot_id** | Option<**String**> | (When copying) The ID of the snapshot you want to copy. | [optional]
**volume_id** | Option<**String**> | (When creating) The ID of the volume you want to create a snapshot of. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


