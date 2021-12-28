# SnapshotExportTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | If the snapshot export task fails, an error message appears. | [optional]
**osu_export** | Option<[**crate::models::OsuExportSnapshotExportTask**](OsuExportSnapshotExportTask.md)> |  | [optional]
**progress** | Option<**i32**> | The progress of the snapshot export task, as a percentage. | [optional]
**snapshot_id** | Option<**String**> | The ID of the snapshot to be exported. | [optional]
**state** | Option<**String**> | The state of the snapshot export task (`pending` \\| `active` \\| `completed` \\| `failed`). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the snapshot export task. | [optional]
**task_id** | Option<**String**> | The ID of the snapshot export task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


