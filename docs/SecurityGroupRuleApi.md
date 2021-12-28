# \SecurityGroupRuleApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group_rule**](SecurityGroupRuleApi.md#create_security_group_rule) | **POST** /CreateSecurityGroupRule | 
[**delete_security_group_rule**](SecurityGroupRuleApi.md#delete_security_group_rule) | **POST** /DeleteSecurityGroupRule | 



## create_security_group_rule

> crate::models::CreateSecurityGroupRuleResponse create_security_group_rule(create_security_group_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_security_group_rule_request** | Option<[**CreateSecurityGroupRuleRequest**](CreateSecurityGroupRuleRequest.md)> |  |  |

### Return type

[**crate::models::CreateSecurityGroupRuleResponse**](CreateSecurityGroupRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group_rule

> crate::models::DeleteSecurityGroupRuleResponse delete_security_group_rule(delete_security_group_rule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_security_group_rule_request** | Option<[**DeleteSecurityGroupRuleRequest**](DeleteSecurityGroupRuleRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSecurityGroupRuleResponse**](DeleteSecurityGroupRuleResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

