# VolumeUpdateTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | If the update volume task fails, an error message appears. | [optional]
**completion_date** | Option<**String**> | The date at which the volume update task was marked as completed. | [optional]
**progress** | Option<**i32**> | The progress of the volume update task, as a percentage. | [optional]
**start_date** | Option<**String**> | The creation date of the volume update task. | [optional]
**state** | Option<**String**> | The state of the volume (`pending` \\| `active` \\| `completed` \\| `failed` \\| `canceled`). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the volume update task. | [optional]
**task_id** | Option<**String**> | The ID of the volume update task in progress. Otherwise, it is not returned. | [optional]
**volume_id** | Option<**String**> | The ID of the updated volume. | [optional]
**volume_update** | Option<[**crate::models::VolumeUpdate**](VolumeUpdate.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


