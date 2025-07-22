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


Creates a rule to allow access to the API from your account.<br /> You need to specify at least the `CaIds` or the `IpRanges` parameter.<br /><br />  **[NOTE]**<br /> By default, your account has a set of rules allowing global access, that you can delete.<br /><br /> For more information, see [About API Access Rules](https://docs.outscale.com/en/userguide/About-API-Access-Rules.html).

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


Deletes a specified API access rule.<br /><br />  **[IMPORTANT]**<br /> You cannot delete the last remaining API access rule. However, if you delete all the API access rules that allow you to access the APIs, you need to contact the Support team to regain access. For more information, see [Technical Support](https://docs.outscale.com/en/userguide/Technical-Support.html).

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


Lists one or more API access rules.

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


Modifies a specified API access rule.<br /><br />  **[WARNING]**<br /> - The new rule you specify fully replaces the old rule. Therefore, for a parameter that is not specified, any previously set value is deleted.<br /> - If, as result of your modification, you no longer have access to the APIs, you will need to contact the Support team to regain access. For more information, see [Technical Support](https://docs.outscale.com/en/userguide/Technical-Support.html).

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

