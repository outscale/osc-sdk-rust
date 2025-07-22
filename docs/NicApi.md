# \NicApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nic**](NicApi.md#create_nic) | **POST** /CreateNic | 
[**delete_nic**](NicApi.md#delete_nic) | **POST** /DeleteNic | 
[**link_nic**](NicApi.md#link_nic) | **POST** /LinkNic | 
[**link_private_ips**](NicApi.md#link_private_ips) | **POST** /LinkPrivateIps | 
[**read_nics**](NicApi.md#read_nics) | **POST** /ReadNics | 
[**unlink_nic**](NicApi.md#unlink_nic) | **POST** /UnlinkNic | 
[**unlink_private_ips**](NicApi.md#unlink_private_ips) | **POST** /UnlinkPrivateIps | 
[**update_nic**](NicApi.md#update_nic) | **POST** /UpdateNic | 



## create_nic

> crate::models::CreateNicResponse create_nic(create_nic_request)


Creates a network interface card (NIC) in the specified Subnet.<br /><br /> For more information, see [About NICs](https://docs.outscale.com/en/userguide/About-NICs.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_nic_request** | Option<[**CreateNicRequest**](CreateNicRequest.md)> |  |  |

### Return type

[**crate::models::CreateNicResponse**](CreateNicResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_nic

> crate::models::DeleteNicResponse delete_nic(delete_nic_request)


Deletes the specified network interface card (NIC).<br /> The network interface must not be attached to any virtual machine (VM).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_nic_request** | Option<[**DeleteNicRequest**](DeleteNicRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNicResponse**](DeleteNicResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_nic

> crate::models::LinkNicResponse link_nic(link_nic_request)


Attaches a network interface card (NIC) to a virtual machine (VM).<br /> The interface and the VM must be in the same Subregion. The VM can be either `running` or `stopped`. The NIC must be in the `available` state. For more information, see [Attaching a NIC to a VM](https://docs.outscale.com/en/userguide/Attaching-a-NIC-to-a-VM.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_nic_request** | Option<[**LinkNicRequest**](LinkNicRequest.md)> |  |  |

### Return type

[**crate::models::LinkNicResponse**](LinkNicResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_private_ips

> crate::models::LinkPrivateIpsResponse link_private_ips(link_private_ips_request)


Assigns one or more secondary private IPs to a specified network interface card (NIC). This action is only available in a Net. The private IPs to be assigned can be added individually using the `PrivateIps` parameter, or you can specify the number of private IPs to be automatically chosen within the Subnet range using the `SecondaryPrivateIpCount` parameter. You can specify only one of these two parameters. If none of these parameters are specified, a private IP is chosen within the Subnet range.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_private_ips_request** | Option<[**LinkPrivateIpsRequest**](LinkPrivateIpsRequest.md)> |  |  |

### Return type

[**crate::models::LinkPrivateIpsResponse**](LinkPrivateIpsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_nics

> crate::models::ReadNicsResponse read_nics(read_nics_request)


Lists one or more network interface cards (NICs).<br /> A NIC is a virtual network interface that you can attach to a virtual machine (VM) in a Net.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_nics_request** | Option<[**ReadNicsRequest**](ReadNicsRequest.md)> |  |  |

### Return type

[**crate::models::ReadNicsResponse**](ReadNicsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_nic

> crate::models::UnlinkNicResponse unlink_nic(unlink_nic_request)


Detaches a network interface card (NIC) from a virtual machine (VM).<br /> The primary NIC cannot be detached.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_nic_request** | Option<[**UnlinkNicRequest**](UnlinkNicRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkNicResponse**](UnlinkNicResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_private_ips

> crate::models::UnlinkPrivateIpsResponse unlink_private_ips(unlink_private_ips_request)


Unassigns one or more secondary private IPs from a network interface card (NIC).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_private_ips_request** | Option<[**UnlinkPrivateIpsRequest**](UnlinkPrivateIpsRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkPrivateIpsResponse**](UnlinkPrivateIpsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_nic

> crate::models::UpdateNicResponse update_nic(update_nic_request)


Modifies the specified network interface card (NIC). You can specify only one attribute at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_nic_request** | Option<[**UpdateNicRequest**](UpdateNicRequest.md)> |  |  |

### Return type

[**crate::models::UpdateNicResponse**](UpdateNicResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

