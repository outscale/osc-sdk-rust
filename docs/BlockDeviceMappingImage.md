# BlockDeviceMappingImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bsu** | Option<[**crate::models::BsuToCreate**](BsuToCreate.md)> |  | [optional]
**device_name** | Option<**String**> | The device name for the volume. For a root device, you must use `/dev/sda1`. For other volumes, you must use `/dev/sdX` or `/dev/xvdX` (where `X` is a letter between `b` and `z`). | [optional]
**virtual_device_name** | Option<**String**> | The name of the virtual device (`ephemeralN`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


