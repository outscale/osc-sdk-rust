# CreateImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | The architecture of the OMI (by default, `i386` if you specified the `FileLocation` or `RootDeviceName` parameter). | [optional]
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingImage>**](BlockDeviceMappingImage.md)> | One or more block device mappings. | [optional]
**description** | Option<**String**> | A description for the new OMI. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**file_location** | Option<**String**> | The pre-signed URL of the OMI manifest file, or the full path to the OMI stored in a bucket. If you specify this parameter, a copy of the OMI is created in your account. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`. | [optional]
**image_name** | Option<**String**> | A unique name for the new OMI.<br /> Constraints: 3-128 alphanumeric characters, underscores (_), spaces ( ), parentheses (()), slashes (/), periods (.), or dashes (-). | [optional]
**no_reboot** | Option<**bool**> | If false, the VM shuts down before creating the OMI and then reboots. If true, the VM does not. | [optional]
**root_device_name** | Option<**String**> | The name of the root device. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`. | [optional]
**source_image_id** | Option<**String**> | The ID of the OMI you want to copy. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`. | [optional]
**source_region_name** | Option<**String**> | The name of the source Region, which must be the same as the Region of your account. | [optional]
**vm_id** | Option<**String**> | The ID of the VM from which you want to create the OMI. You must specify only one of the following parameters: `FileLocation`, `RootDeviceName`, `SourceImageId` or `VmId`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


