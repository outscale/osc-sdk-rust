# \DedicatedGroupApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dedicated_group**](DedicatedGroupApi.md#create_dedicated_group) | **POST** /CreateDedicatedGroup | 
[**delete_dedicated_group**](DedicatedGroupApi.md#delete_dedicated_group) | **POST** /DeleteDedicatedGroup | 
[**read_dedicated_groups**](DedicatedGroupApi.md#read_dedicated_groups) | **POST** /ReadDedicatedGroups | 
[**update_dedicated_group**](DedicatedGroupApi.md#update_dedicated_group) | **POST** /UpdateDedicatedGroup | 



## create_dedicated_group

> crate::models::CreateDedicatedGroupResponse create_dedicated_group(create_dedicated_group_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dedicated_group_request** | Option<[**CreateDedicatedGroupRequest**](CreateDedicatedGroupRequest.md)> |  |  |

### Return type

[**crate::models::CreateDedicatedGroupResponse**](CreateDedicatedGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dedicated_group

> crate::models::DeleteDedicatedGroupResponse delete_dedicated_group(delete_dedicated_group_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_dedicated_group_request** | Option<[**DeleteDedicatedGroupRequest**](DeleteDedicatedGroupRequest.md)> |  |  |

### Return type

[**crate::models::DeleteDedicatedGroupResponse**](DeleteDedicatedGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_dedicated_groups

> crate::models::ReadDedicatedGroupsResponse read_dedicated_groups(read_dedicated_groups_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_dedicated_groups_request** | Option<[**ReadDedicatedGroupsRequest**](ReadDedicatedGroupsRequest.md)> |  |  |

### Return type

[**crate::models::ReadDedicatedGroupsResponse**](ReadDedicatedGroupsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dedicated_group

> crate::models::UpdateDedicatedGroupResponse update_dedicated_group(update_dedicated_group_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_dedicated_group_request** | Option<[**UpdateDedicatedGroupRequest**](UpdateDedicatedGroupRequest.md)> |  |  |

### Return type

[**crate::models::UpdateDedicatedGroupResponse**](UpdateDedicatedGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

