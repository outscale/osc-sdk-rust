# PolicyEntities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accounts** | Option<[**Vec<crate::models::MinimalPolicy>**](MinimalPolicy.md)> | The accounts linked to the specified policy. | [optional]
**groups** | Option<[**Vec<crate::models::MinimalPolicy>**](MinimalPolicy.md)> | The groups linked to the specified policy. | [optional]
**has_more_items** | Option<**bool**> | If true, there are more items to return using the `FirstItem` parameter in a new request. | [optional]
**items_count** | Option<**i32**> | The number of entities the specified policy is linked to. | [optional]
**max_results_limit** | Option<**i32**> | Indicates maximum results defined for the operation. | [optional]
**max_results_truncated** | Option<**bool**> | If true, indicates whether requested page size is more than allowed. | [optional]
**users** | Option<[**Vec<crate::models::MinimalPolicy>**](MinimalPolicy.md)> | The users linked to the specified policy. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


