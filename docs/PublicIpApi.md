# \PublicIpApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_public_ip**](PublicIpApi.md#create_public_ip) | **POST** /CreatePublicIp | 
[**delete_public_ip**](PublicIpApi.md#delete_public_ip) | **POST** /DeletePublicIp | 
[**link_public_ip**](PublicIpApi.md#link_public_ip) | **POST** /LinkPublicIp | 
[**read_public_ip_ranges**](PublicIpApi.md#read_public_ip_ranges) | **POST** /ReadPublicIpRanges | 
[**read_public_ips**](PublicIpApi.md#read_public_ips) | **POST** /ReadPublicIps | 
[**unlink_public_ip**](PublicIpApi.md#unlink_public_ip) | **POST** /UnlinkPublicIp | 



## create_public_ip

> crate::models::CreatePublicIpResponse create_public_ip(create_public_ip_request)


Acquires a public IP for your account.<br /> A public IP is a static IP designed for dynamic Cloud computing. It can be associated with a virtual machine (VM) in the public Cloud or in a Net, a network interface card (NIC), a NAT service.<br /><br /> For more information, see [About Public IPs](https://docs.outscale.com/en/userguide/About-Public-IPs.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_public_ip_request** | Option<[**CreatePublicIpRequest**](CreatePublicIpRequest.md)> |  |  |

### Return type

[**crate::models::CreatePublicIpResponse**](CreatePublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_public_ip

> crate::models::DeletePublicIpResponse delete_public_ip(delete_public_ip_request)


Releases a public IP.<br /> You can release a public IP associated with your account. This address is released in the public IP pool and can be used by someone else. Before releasing a public IP, ensure you updated all your resources communicating with this address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_public_ip_request** | Option<[**DeletePublicIpRequest**](DeletePublicIpRequest.md)> |  |  |

### Return type

[**crate::models::DeletePublicIpResponse**](DeletePublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_public_ip

> crate::models::LinkPublicIpResponse link_public_ip(link_public_ip_request)


Associates a public IP with a virtual machine (VM) or a network interface card (NIC), in the public Cloud or in a Net. You can associate a public IP with only one VM or network interface at a time.<br /> To associate a public IP in a Net, ensure that the Net has an internet service attached. For more information, see the [LinkInternetService](#linkinternetservice) method.<br /> By default, the public IP is disassociated every time you stop and start the VM. For a persistent association, you can add the `osc.fcu.eip.auto-attach` tag to the VM with the public IP as value. For more information, see the [CreateTags](#createtags) method.<br /><br />  **[IMPORTANT]**<br /> You can associate a public IP with a network address translation (NAT) service only when creating the NAT service. To modify its public IP, you need to delete the NAT service and re-create it with the new public IP. For more information, see the [CreateNatService](#createnatservice) method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_public_ip_request** | Option<[**LinkPublicIpRequest**](LinkPublicIpRequest.md)> |  |  |

### Return type

[**crate::models::LinkPublicIpResponse**](LinkPublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_public_ip_ranges

> crate::models::ReadPublicIpRangesResponse read_public_ip_ranges(read_public_ip_ranges_request)


Gets the public IPv4 addresses in CIDR notation for the Region specified in the endpoint of the request. For more information, see [About Regions and Subregions](https://docs.outscale.com/en/userguide/About-Regions-and-Subregions.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_public_ip_ranges_request** | Option<[**ReadPublicIpRangesRequest**](ReadPublicIpRangesRequest.md)> |  |  |

### Return type

[**crate::models::ReadPublicIpRangesResponse**](ReadPublicIpRangesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_public_ips

> crate::models::ReadPublicIpsResponse read_public_ips(read_public_ips_request)


Lists one or more public IPs allocated to your account.<br /> By default, this action returns information about all your public IPs: available or associated with a virtual machine (VM), a network interface card (NIC) or a NAT service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_public_ips_request** | Option<[**ReadPublicIpsRequest**](ReadPublicIpsRequest.md)> |  |  |

### Return type

[**crate::models::ReadPublicIpsResponse**](ReadPublicIpsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_public_ip

> crate::models::UnlinkPublicIpResponse unlink_public_ip(unlink_public_ip_request)


Disassociates a public IP from the virtual machine (VM) or network interface card (NIC) it is associated with.<br /><br />  **[IMPORTANT]**<br /> To disassociate the public IP from a NAT service, you need to delete the NAT service. For more information, see the [DeleteNatService](#deletenatservice) method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_public_ip_request** | Option<[**UnlinkPublicIpRequest**](UnlinkPublicIpRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkPublicIpResponse**](UnlinkPublicIpResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

