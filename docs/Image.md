# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_alias** | Option<**String**> | The account alias of the owner of the OMI. | [optional]
**account_id** | Option<**String**> | The account ID of the owner of the OMI. | [optional]
**architecture** | Option<**String**> | The architecture of the OMI. | [optional]
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingImage>**](BlockDeviceMappingImage.md)> | One or more block device mappings. | [optional]
**creation_date** | Option<[**String**](string.md)> | The date and time (UTC) at which the OMI was created. | [optional]
**description** | Option<**String**> | The description of the OMI. | [optional]
**file_location** | Option<**String**> | The location from which the OMI files were created. | [optional]
**image_id** | Option<**String**> | The ID of the OMI. | [optional]
**image_name** | Option<**String**> | The name of the OMI. | [optional]
**image_type** | Option<**String**> | The type of the OMI. | [optional]
**permissions_to_launch** | Option<[**crate::models::PermissionsOnResource**](PermissionsOnResource.md)> |  | [optional]
**product_codes** | Option<**Vec<String>**> | The product codes associated with the OMI. | [optional]
**root_device_name** | Option<**String**> | The name of the root device. | [optional]
**root_device_type** | Option<**String**> | The type of root device used by the OMI (always `bsu`). | [optional]
**state** | Option<**String**> | The state of the OMI (`pending` \\| `available` \\| `failed`). | [optional]
**state_comment** | Option<[**crate::models::StateComment**](StateComment.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the OMI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


