# \ApiAccessPolicyApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**read_api_access_policy**](ApiAccessPolicyApi.md#read_api_access_policy) | **POST** /ReadApiAccessPolicy | 
[**update_api_access_policy**](ApiAccessPolicyApi.md#update_api_access_policy) | **POST** /UpdateApiAccessPolicy | 



## read_api_access_policy

> crate::models::ReadApiAccessPolicyResponse read_api_access_policy(read_api_access_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_api_access_policy_request** | Option<[**ReadApiAccessPolicyRequest**](ReadApiAccessPolicyRequest.md)> |  |  |

### Return type

[**crate::models::ReadApiAccessPolicyResponse**](ReadApiAccessPolicyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_access_policy

> crate::models::UpdateApiAccessPolicyResponse update_api_access_policy(update_api_access_policy_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_api_access_policy_request** | Option<[**UpdateApiAccessPolicyRequest**](UpdateApiAccessPolicyRequest.md)> |  |  |

### Return type

[**crate::models::UpdateApiAccessPolicyResponse**](UpdateApiAccessPolicyResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

