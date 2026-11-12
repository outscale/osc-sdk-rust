# UpdateApiAccessPolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**max_access_key_expiration_seconds** | **i64** | The maximum possible lifetime for your access keys, in seconds (between `0` and `3153600000`, both included). If set to `O`, your access keys can have unlimited lifetimes, but a trusted session cannot be activated. Otherwise, all your access keys must have an expiration date. This value must be greater than the remaining lifetime of each access key of your account. | 
**require_trusted_env** | **bool** | If true, a trusted session is activated, provided that you specify the `MaxAccessKeyExpirationSeconds` parameter with a value greater than `0`. This allows you to bypass Certificate Authorities (CAs) enforcement. | 
**require_trusted_env_secure_mode** | **bool** | If true, a trusted session is activated, provided that you specify the `MaxAccessKeyExpirationSeconds` parameter with a value greater than `0` and that the parameter `RequireTrustedEnv` is set to `true`. This requires you and all your EIM users to use only trusted access keys, to log in to Cockpit v2 using multi-factor authentication (MFA), and will restrict the CLI use of certain API calls. For more information, see [About Your API Access Policy > Scope of a Trusted Session](https://docs.outscale.com/en/userguide/About-Your-API-Access-Policy.html#_scope_of_a_trusted_session). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


