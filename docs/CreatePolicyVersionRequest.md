# CreatePolicyVersionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**document** | **String** | The policy document, corresponding to a JSON string that contains the policy. This policy document can contain a maximum of 5120 non-whitespace characters. For more information, see [EIM Reference Information](https://docs.outscale.com/en/userguide/EIM-Reference-Information.html) and [EIM Policy Generator](https://docs.outscale.com/en/userguide/EIM-Policy-Generator.html). | 
**policy_orn** | **String** | The OUTSCALE Resource Name (ORN) of the policy. For more information, see [Resource Identifiers](https://docs.outscale.com/en/userguide/Resource-Identifiers.html). | 
**set_as_default** | Option<**bool**> | If set to true, the new policy version is set as the default version and becomes the operative one. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


