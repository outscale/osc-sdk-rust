# FiltersApiLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query_access_keys** | Option<**Vec<String>**> | The access keys used for the logged calls. | [optional]
**query_api_names** | Option<**Vec<String>**> | The names of the APIs of the logged calls (always `oapi` for the OUTSCALE API). | [optional]
**query_call_names** | Option<**Vec<String>**> | The names of the logged calls. | [optional]
**query_date_after** | Option<[**String**](string.md)> | The date after which you want to retrieve logged calls, in ISO 8601 format (for example, `2020-06-14`). By default, this date is set to 48 hours before the `QueryDateBefore` parameter value. | [optional]
**query_date_before** | Option<[**String**](string.md)> | The date before which you want to retrieve logged calls, in ISO 8601 format (for example, `2020-06-30`). By default, this date is set to now, or 48 hours after the `QueryDateAfter` parameter value. | [optional]
**query_ip_addresses** | Option<**Vec<String>**> | The IPs used for the logged calls. | [optional]
**query_user_agents** | Option<**Vec<String>**> | The user agents of the HTTP requests of the logged calls. | [optional]
**request_ids** | Option<**Vec<String>**> | The request IDs provided in the responses of the logged calls. | [optional]
**response_status_codes** | Option<**Vec<i32>**> | The HTTP status codes of the logged calls. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


