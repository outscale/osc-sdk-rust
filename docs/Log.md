# Log

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the logged call. | [optional]
**call_duration** | Option<**i32**> | The duration of the logged call, in microseconds. | [optional]
**query_access_key** | Option<**String**> | The access key used for the logged call. | [optional]
**query_api_name** | Option<**String**> | The name of the API used by the logged call (always `oapi` for the OUTSCALE API). | [optional]
**query_api_version** | Option<**String**> | The version of the API used by the logged call. | [optional]
**query_call_name** | Option<**String**> | The name of the logged call. | [optional]
**query_date** | Option<[**String**](string.md)> | The date of the logged call, in ISO 8601 format. | [optional]
**query_header_raw** | Option<**String**> | The raw header of the HTTP request of the logged call. | [optional]
**query_header_size** | Option<**i32**> | The size of the raw header of the HTTP request of the logged call, in bytes. | [optional]
**query_ip_address** | Option<**String**> | The IP used for the logged call. | [optional]
**query_payload_raw** | Option<**String**> | The raw payload of the HTTP request of the logged call. | [optional]
**query_payload_size** | Option<**i32**> | The size of the raw payload of the HTTP request of the logged call, in bytes. | [optional]
**query_user_agent** | Option<**String**> | The user agent of the HTTP request of the logged call. | [optional]
**request_id** | Option<**String**> | The request ID provided in the response of the logged call. | [optional]
**response_size** | Option<**i32**> | The size of the response of the logged call, in bytes. | [optional]
**response_status_code** | Option<**i32**> | The HTTP status code of the response of the logged call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


