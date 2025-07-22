# \DhcpOptionApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dhcp_options**](DhcpOptionApi.md#create_dhcp_options) | **POST** /CreateDhcpOptions | 
[**delete_dhcp_options**](DhcpOptionApi.md#delete_dhcp_options) | **POST** /DeleteDhcpOptions | 
[**read_dhcp_options**](DhcpOptionApi.md#read_dhcp_options) | **POST** /ReadDhcpOptions | 



## create_dhcp_options

> crate::models::CreateDhcpOptionsResponse create_dhcp_options(create_dhcp_options_request)


Creates a set of DHCP options, that you can then associate with a Net using the [UpdateNet](#updatenet) method.<br /><br /> For more information, see [About DHCP Options](https://docs.outscale.com/en/userguide/About-DHCP-Options.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_dhcp_options_request** | Option<[**CreateDhcpOptionsRequest**](CreateDhcpOptionsRequest.md)> |  |  |

### Return type

[**crate::models::CreateDhcpOptionsResponse**](CreateDhcpOptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dhcp_options

> crate::models::DeleteDhcpOptionsResponse delete_dhcp_options(delete_dhcp_options_request)


Deletes a specified DHCP options set.<br /> Before deleting a DHCP options set, you must disassociate it from the Nets you associated it with. To do so, you need to associate with each Net a new set of DHCP options, or the `default` one if you do not want to associate any DHCP options with the Net.<br /><br />  **[IMPORTANT]**<br /> You cannot delete the `default` set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_dhcp_options_request** | Option<[**DeleteDhcpOptionsRequest**](DeleteDhcpOptionsRequest.md)> |  |  |

### Return type

[**crate::models::DeleteDhcpOptionsResponse**](DeleteDhcpOptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_dhcp_options

> crate::models::ReadDhcpOptionsResponse read_dhcp_options(read_dhcp_options_request)


Gets information about the content of one or more DHCP options sets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_dhcp_options_request** | Option<[**ReadDhcpOptionsRequest**](ReadDhcpOptionsRequest.md)> |  |  |

### Return type

[**crate::models::ReadDhcpOptionsResponse**](ReadDhcpOptionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

