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


> [WARNING]<br /> > This feature is currently in beta.<br />  Creates a dedicated group for virtual machines (VMs).<br /><br /> For more information, see [About Dedicated Groups](https://docs.outscale.com/en/userguide/About-Dedicated-Groups.html).

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


> [WARNING]<br /> > This feature is currently in beta.<br />  Deletes a specified dedicated group of virtual machines (VMs).<br />  **[WARNING]**<br /> A dedicated group can be deleted only if no VM or Net is in the dedicated group. Otherwise, you need to force the deletion.<br /> If you force the deletion:<br /> - all VMs are terminated.<br /> - all Nets are deleted, and all resources associated with Nets are detached.

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


> [WARNING]<br /> > This feature is currently in beta.<br />  List one or more dedicated groups of virtual machines (VMs).

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


> [WARNING]<br /> > This feature is currently in beta.<br />  Modifies the name of a specified dedicated group.<br />

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

