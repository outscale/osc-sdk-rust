# LinkVolumeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_name** | **String** | The name of the device. For a root device, you must use `/dev/sda1`. For other volumes, you must use `/dev/sdX`, `/dev/sdXY`, `/dev/xvdX`, or `/dev/xvdXY` (where `X` is a letter between `b` and `z` and where `Y` is a letter between `a` and `z`). | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**vm_id** | **String** | The ID of the VM you want to attach the volume to. | 
**volume_id** | **String** | The ID of the volume you want to attach. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


