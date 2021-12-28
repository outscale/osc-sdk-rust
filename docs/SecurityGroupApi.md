# \SecurityGroupApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_security_group**](SecurityGroupApi.md#create_security_group) | **POST** /CreateSecurityGroup | 
[**delete_security_group**](SecurityGroupApi.md#delete_security_group) | **POST** /DeleteSecurityGroup | 
[**read_security_groups**](SecurityGroupApi.md#read_security_groups) | **POST** /ReadSecurityGroups | 



## create_security_group

> crate::models::CreateSecurityGroupResponse create_security_group(create_security_group_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_security_group_request** | Option<[**CreateSecurityGroupRequest**](CreateSecurityGroupRequest.md)> |  |  |

### Return type

[**crate::models::CreateSecurityGroupResponse**](CreateSecurityGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_security_group

> crate::models::DeleteSecurityGroupResponse delete_security_group(delete_security_group_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_security_group_request** | Option<[**DeleteSecurityGroupRequest**](DeleteSecurityGroupRequest.md)> |  |  |

### Return type

[**crate::models::DeleteSecurityGroupResponse**](DeleteSecurityGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_security_groups

> crate::models::ReadSecurityGroupsResponse read_security_groups(read_security_groups_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_security_groups_request** | Option<[**ReadSecurityGroupsRequest**](ReadSecurityGroupsRequest.md)> |  |  |

### Return type

[**crate::models::ReadSecurityGroupsResponse**](ReadSecurityGroupsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

