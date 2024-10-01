# CreateImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | **(when registering from a snapshot)** The architecture of the OMI (`i386` or `x86_64`). | [optional]
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingImage>**](BlockDeviceMappingImage.md)> | **(when registering from a snapshot)** One or more block device mappings. | [optional]
**description** | Option<**String**> | A description for the new OMI. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**file_location** | Option<**String**> | **(when registering from a bucket by using a manifest file)** The pre-signed URL of the manifest file for the OMI you want to register. For more information, see [Creating a Pre-signed URL](https://docs.outscale.com/en/userguide/Creating-a-Pre-Signed-URL.html). | [optional]
**image_name** | Option<**String**> | A unique name for the new OMI.<br /> Constraints: 3-128 alphanumeric characters, underscores (`_`), spaces (` `), parentheses (`()`), slashes (`/`), periods (`.`), or dashes (`-`). | [optional]
**no_reboot** | Option<**bool**> | **(when creating from a VM)** If false, the VM shuts down before creating the OMI and then reboots. If true, the VM does not. | [optional]
**product_codes** | Option<**Vec<String>**> | The product codes associated with the OMI. | [optional]
**root_device_name** | Option<**String**> | **(when registering from a snapshot)** The name of the root device for the new OMI. | [optional]
**source_image_id** | Option<**String**> | **(when copying an OMI)** The ID of the OMI you want to copy. | [optional]
**source_region_name** | Option<**String**> | **(when copying an OMI)** The name of the source Region (always the same as the Region of your account). | [optional]
**vm_id** | Option<**String**> | **(when creating from a VM)** The ID of the VM from which you want to create the OMI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


