# ReadUsersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**has_more_items** | Option<**bool**> | If true, there are more items to return using the `FirstItem` parameter in a new request. | [optional]
**max_results_limit** | Option<**i32**> | Indicates maximum results defined for the operation. | [optional]
**max_results_truncated** | Option<**bool**> | If true, indicates whether requested page size is more than allowed. | [optional]
**response_context** | Option<[**crate::models::ResponseContext**](ResponseContext.md)> |  | [optional]
**users** | Option<[**Vec<crate::models::User>**](User.md)> | A list of EIM users. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


