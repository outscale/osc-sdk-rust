# ApiAccessPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_access_key_expiration_seconds** | Option<**i64**> | The maximum possible lifetime for your access keys, in seconds. If `0`, your access keys can have unlimited lifetimes. | [optional]
**require_trusted_env** | Option<**bool**> | If true, a trusted session is activated, allowing you to bypass Certificate Authorities (CAs) enforcement. For more information, see the `ApiKeyAuth` authentication scheme in the [Authentication](#authentication) section. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


