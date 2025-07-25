# \VmGroupApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vm_group**](VmGroupApi.md#create_vm_group) | **POST** /CreateVmGroup | 
[**delete_vm_group**](VmGroupApi.md#delete_vm_group) | **POST** /DeleteVmGroup | 
[**read_vm_groups**](VmGroupApi.md#read_vm_groups) | **POST** /ReadVmGroups | 
[**scale_down_vm_group**](VmGroupApi.md#scale_down_vm_group) | **POST** /ScaleDownVmGroup | 
[**scale_up_vm_group**](VmGroupApi.md#scale_up_vm_group) | **POST** /ScaleUpVmGroup | 
[**update_vm_group**](VmGroupApi.md#update_vm_group) | **POST** /UpdateVmGroup | 



## create_vm_group

> crate::models::CreateVmGroupResponse create_vm_group(create_vm_group_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Creates a group of virtual machines (VMs) containing the same characteristics as a specified VM template, and then launches them.<br /> You can create up to 100 VM groups in your account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vm_group_request** | Option<[**CreateVmGroupRequest**](CreateVmGroupRequest.md)> |  |  |

### Return type

[**crate::models::CreateVmGroupResponse**](CreateVmGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vm_group

> crate::models::DeleteVmGroupResponse delete_vm_group(delete_vm_group_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Deletes a specified VM group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_vm_group_request** | Option<[**DeleteVmGroupRequest**](DeleteVmGroupRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVmGroupResponse**](DeleteVmGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vm_groups

> crate::models::ReadVmGroupsResponse read_vm_groups(read_vm_groups_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Lists one or more group of virtual machines (VMs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vm_groups_request** | Option<[**ReadVmGroupsRequest**](ReadVmGroupsRequest.md)> |  |  |

### Return type

[**crate::models::ReadVmGroupsResponse**](ReadVmGroupsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_down_vm_group

> crate::models::ScaleDownVmGroupResponse scale_down_vm_group(scale_down_vm_group_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Deletes virtual machines (VMs) from a VM group.<br /> The oldest VMs are the first to be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scale_down_vm_group_request** | Option<[**ScaleDownVmGroupRequest**](ScaleDownVmGroupRequest.md)> |  |  |

### Return type

[**crate::models::ScaleDownVmGroupResponse**](ScaleDownVmGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scale_up_vm_group

> crate::models::ScaleUpVmGroupResponse scale_up_vm_group(scale_up_vm_group_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Creates additional virtual machines (VMs) in a VM group.<br /> The new VMs use the current version of the VM template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scale_up_vm_group_request** | Option<[**ScaleUpVmGroupRequest**](ScaleUpVmGroupRequest.md)> |  |  |

### Return type

[**crate::models::ScaleUpVmGroupResponse**](ScaleUpVmGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vm_group

> crate::models::UpdateVmGroupResponse update_vm_group(update_vm_group_request)


> [WARNING]<br /> > This feature is currently under development and may not function properly.<br />  Modifies the specified attributes of a group of virtual machines (VMs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_vm_group_request** | Option<[**UpdateVmGroupRequest**](UpdateVmGroupRequest.md)> |  |  |

### Return type

[**crate::models::UpdateVmGroupResponse**](UpdateVmGroupResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

