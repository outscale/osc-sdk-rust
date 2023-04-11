# UpdateAccessKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_key_id** | **String** | The ID of the access key. | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**expiration_date** | Option<**String**> | The date and time, or the date, at which you want the access key to expire, in ISO 8601 format (for example, `2020-06-14T00:00:00.000Z` or `2020-06-14`). If not specified, the access key is set to not expire. | [optional]
**state** | **String** | The new state for the access key (`ACTIVE` \\| `INACTIVE`). When set to `ACTIVE`, the access key is enabled and can be used to send requests. When set to `INACTIVE`, the access key is disabled. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


