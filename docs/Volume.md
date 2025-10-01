# Volume

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_token** | Option<**String**> | The idempotency token provided when creating the volume. | [optional]
**creation_date** | Option<**String**> | The date and time (UTC) at which the volume was created. | [optional]
**iops** | Option<**i32**> | The number of I/O operations per second (IOPS):<br /> - For `io1` volumes, the number of provisioned IOPS<br /> - For `gp2` volumes, the baseline performance of the volume | [optional]
**linked_volumes** | Option<[**Vec<crate::models::LinkedVolume>**](LinkedVolume.md)> | Information about your volume attachment. | [optional]
**size** | Option<**i32**> | The size of the volume, in gibibytes (GiB). | [optional]
**snapshot_id** | Option<**String**> | The snapshot from which the volume was created. | [optional]
**state** | Option<**String**> | The state of the volume (`creating` \\| `available` \\| `in-use` \\| `deleting` \\| `error`). | [optional]
**subregion_name** | Option<**String**> | The Subregion in which the volume was created. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the volume. | [optional]
**task_id** | Option<**String**> | The ID of the volume update task in progress. Otherwise, it is not returned. | [optional]
**volume_id** | Option<**String**> | The ID of the volume. | [optional]
**volume_type** | Option<**String**> | The type of the volume (`standard` \\| `gp2` \\| `io1`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


