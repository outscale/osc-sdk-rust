# UpdateAccessKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | **String** | The ID of the access key. | 
**clear_expiration_date** | Option<**bool**> | If true, the current expiration date is deleted and the access key is set to not expire. | [optional]
**clear_tag** | Option<**bool**> | If true, the current tag of the access key is deleted. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**expiration_date** | Option<**String**> | The date and time, or the date, at which you want the access key to expire, in ISO 8601 format (for example, `2020-06-14T00:00:00.000Z` or `2020-06-14`). If not specified, the access key is set to not expire. If the `ClearExpirationDate` parameter is set to true, the expiration date is ignored. | [optional]
**state** | Option<**String**> | The new state for the access key (`ACTIVE` \\| `INACTIVE`). When set to `ACTIVE`, the access key is enabled and can be used to send requests. When set to `INACTIVE`, the access key is disabled. | [optional]
**tag** | Option<**String**> | A new tag to add to the access key. If the access key already had a tag, this replaces it. If the `ClearTag` parameter is set to true, the tag is ignored. | [optional]
**user_name** | Option<**String**> | The name of the EIM user that the access key you want to modify is associated with. If you do not specify a user name, this action modifies the access key of the user who sends the request (which can be the root user). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


