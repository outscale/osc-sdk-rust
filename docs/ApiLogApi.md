# \ApiLogApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_api_logs**](ApiLogApi.md#read_api_logs) | **POST** /ReadApiLogs | 



## read_api_logs

> crate::models::ReadApiLogsResponse read_api_logs(read_api_logs_request)


Lists the logs of the API calls you have performed with this account.  **[IMPORTANT]**<br /> Past logs are accessible for up to 32 days.<br /> By default, the retrieved interval is 48 hours. If neither of the `QueryDateBefore` nor `QueryDateAfter` parameters are specified, logs from the past 48 hours are retrieved. If you only specify one of two, logs are retrieved from a 2-day interval based on the date you provided. To retrieve logs beyond a 2-day interval, specify both parameters.<br /><br /> For more information, see [About OMS](https://docs.outscale.com/en/userguide/About-OMS.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_api_logs_request** | Option<[**ReadApiLogsRequest**](ReadApiLogsRequest.md)> |  |  |

### Return type

[**crate::models::ReadApiLogsResponse**](ReadApiLogsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

