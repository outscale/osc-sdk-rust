# CreateAccessKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**expiration_date** | Option<**String**> | The date and time, or the date, at which you want the access key to expire, in ISO 8601 format (for example, `2020-06-14T00:00:00.000Z`, or `2020-06-14`). To remove an existing expiration date, use the method without specifying this parameter. | [optional]
**tag** | Option<**String**> | A tag to add to the access key. | [optional]
**user_name** | Option<**String**> | The name of the EIM user that owns the key to be created. If you do not specify a user name, this action creates an access key for the user who sends the request (which can be the root account). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


