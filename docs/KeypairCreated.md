# KeypairCreated

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**keypair_fingerprint** | Option<**String**> | The MD5 public key fingerprint, as specified in section 4 of RFC 4716. | [optional]
**keypair_id** | Option<**String**> | The ID of the keypair. | [optional]
**keypair_name** | Option<**String**> | The name of the keypair. | [optional]
**keypair_type** | Option<**String**> | The type of the keypair (`ssh-rsa`, `ssh-ed25519`, `ecdsa-sha2-nistp256`, `ecdsa-sha2-nistp384`, or `ecdsa-sha2-nistp521`). | [optional]
**private_key** | Option<**String**> | The private key, returned only if you are creating a keypair (not if you are importing). When you save this private key in a .rsa file, make sure you replace the `\\n` escape sequences with real line breaks. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the keypair. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


