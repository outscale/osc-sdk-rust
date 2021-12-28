# ImageExportTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | If the OMI export task fails, an error message appears. | [optional]
**image_id** | Option<**String**> | The ID of the OMI to be exported. | [optional]
**osu_export** | Option<[**crate::models::OsuExportImageExportTask**](OsuExportImageExportTask.md)> |  | [optional]
**progress** | Option<**i32**> | The progress of the OMI export task, as a percentage. | [optional]
**state** | Option<**String**> | The state of the OMI export task (`pending/queued` \\| `pending` \\| `completed` \\| `failed` \\| `cancelled`). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the image export task. | [optional]
**task_id** | Option<**String**> | The ID of the OMI export task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


