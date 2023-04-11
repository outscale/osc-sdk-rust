# FiltersApiLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_access_keys** | Option<**Vec<String>**> | The access keys used for the logged calls. | [optional]
**query_api_names** | Option<**Vec<String>**> | The names of the APIs of the logged calls (always `oapi` for the OUTSCALE API). | [optional]
**query_call_names** | Option<**Vec<String>**> | The names of the logged calls. | [optional]
**query_date_after** | Option<[**crate::models::FiltersApiLogQueryDateAfter**](FiltersApiLog_QueryDateAfter.md)> |  | [optional]
**query_date_before** | Option<[**crate::models::FiltersApiLogQueryDateBefore**](FiltersApiLog_QueryDateBefore.md)> |  | [optional]
**query_ip_addresses** | Option<**Vec<String>**> | The IPs used for the logged calls. | [optional]
**query_user_agents** | Option<**Vec<String>**> | The user agents of the HTTP requests of the logged calls. | [optional]
**request_ids** | Option<**Vec<String>**> | The request IDs provided in the responses of the logged calls. | [optional]
**response_status_codes** | Option<**Vec<i32>**> | The HTTP status codes of the logged calls. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


