# ReadPolicyVersionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_more_items** | Option<**bool**> | If true, there are more items to return using the `FirstItem` parameter in a new request. | [optional]
**max_results_limit** | Option<**i32**> | Indicates maximum results defined for the operation. | [optional]
**policy_versions** | Option<[**Vec<crate::models::PolicyVersion>**](PolicyVersion.md)> | A list of all the versions of the policy. | [optional]
**response_context** | Option<[**crate::models::ResponseContext**](ResponseContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


