# UpdateImageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A new description for the image. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**image_id** | **String** | The ID of the OMI you want to modify. | 
**permissions_to_launch** | Option<[**crate::models::PermissionsOnResourceCreation**](PermissionsOnResourceCreation.md)> |  | [optional]
**product_codes** | Option<**Vec<String>**> | The product codes associated with the OMI. Any previously set value is deleted. Make sure to specify all product codes you want to associate with the OMI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


