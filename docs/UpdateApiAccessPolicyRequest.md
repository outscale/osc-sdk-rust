# UpdateApiAccessPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**max_access_key_expiration_seconds** | **i64** | The maximum possible lifetime for your access keys, in seconds (between `0` and `3153600000`, both included). By default or if set to `O`, your access keys can have unlimited lifetimes. Otherwise, all your access keys must have an expiration date. This value must be greater than the remaining lifetime of each access key of your account. | 
**require_trusted_env** | **bool** | If true, a trusted session is activated, provided that you specify the `MaxAccessKeyExpirationSeconds` parameter with a value greater than `0`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


