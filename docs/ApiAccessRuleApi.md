# \ApiAccessRuleApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_access_rule**](ApiAccessRuleApi.md#create_api_access_rule) | **POST** /CreateApiAccessRule | 
[**delete_api_access_rule**](ApiAccessRuleApi.md#delete_api_access_rule) | **POST** /DeleteApiAccessRule | 
[**read_api_access_rules**](ApiAccessRuleApi.md#read_api_access_rules) | **POST** /ReadApiAccessRules | 
[**update_api_access_rule**](ApiAccessRuleApi.md#update_api_access_rule) | **POST** /UpdateApiAccessRule | 



## create_api_access_rule

> crate::models::CreateApiAccessRuleResponse create_api_access_rule(create_api_access_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_access_rule_request** | Option<[**CreateApiAccessRuleRequest**](CreateApiAccessRuleRequest.md)> |  |  |

### Return type

[**crate::models::CreateApiAccessRuleResponse**](CreateApiAccessRuleResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_access_rule

> crate::models::DeleteApiAccessRuleResponse delete_api_access_rule(delete_api_access_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_api_access_rule_request** | Option<[**DeleteApiAccessRuleRequest**](DeleteApiAccessRuleRequest.md)> |  |  |

### Return type

[**crate::models::DeleteApiAccessRuleResponse**](DeleteApiAccessRuleResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_api_access_rules

> crate::models::ReadApiAccessRulesResponse read_api_access_rules(read_api_access_rules_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_api_access_rules_request** | Option<[**ReadApiAccessRulesRequest**](ReadApiAccessRulesRequest.md)> |  |  |

### Return type

[**crate::models::ReadApiAccessRulesResponse**](ReadApiAccessRulesResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_api_access_rule

> crate::models::UpdateApiAccessRuleResponse update_api_access_rule(update_api_access_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_api_access_rule_request** | Option<[**UpdateApiAccessRuleRequest**](UpdateApiAccessRuleRequest.md)> |  |  |

### Return type

[**crate::models::UpdateApiAccessRuleResponse**](UpdateApiAccessRuleResponse.md)

### Authorization

[ApiKeyAuthSec](../README.md#ApiKeyAuthSec), [BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

