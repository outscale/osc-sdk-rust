# ApiAccessPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_access_key_expiration_seconds** | Option<**i64**> | The maximum possible lifetime for your access keys, in seconds. If `0`, your access keys can have unlimited lifetimes. | [optional]
**require_trusted_env** | Option<**bool**> | If true, a trusted session is activated, allowing you to bypass Certificate Authorities (CAs) enforcement. For more information, see [About Your API Access Policy](https://docs.outscale.com/en/userguide/About-Your-API-Access-Policy.html). | [optional]
**require_trusted_env_secure_mode** | Option<**bool**> | If true, a trusted session is activated. This will require you and all your EIM users to use only trusted access keys, to log in to Cockpit v2 using multi-factor authentication (MFA), and will restrict the CLI use of certain API calls. For more information, see [About Your API Access Policy](https://docs.outscale.com/en/userguide/About-Your-API-Access-Policy.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


