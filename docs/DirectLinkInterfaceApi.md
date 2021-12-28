# \DirectLinkInterfaceApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_direct_link_interface**](DirectLinkInterfaceApi.md#create_direct_link_interface) | **POST** /CreateDirectLinkInterface | 
[**delete_direct_link_interface**](DirectLinkInterfaceApi.md#delete_direct_link_interface) | **POST** /DeleteDirectLinkInterface | 
[**read_direct_link_interfaces**](DirectLinkInterfaceApi.md#read_direct_link_interfaces) | **POST** /ReadDirectLinkInterfaces | 



## create_direct_link_interface

> crate::models::CreateDirectLinkInterfaceResponse create_direct_link_interface(create_direct_link_interface_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_direct_link_interface_request** | Option<[**CreateDirectLinkInterfaceRequest**](CreateDirectLinkInterfaceRequest.md)> |  |  |

### Return type

[**crate::models::CreateDirectLinkInterfaceResponse**](CreateDirectLinkInterfaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_direct_link_interface

> crate::models::DeleteDirectLinkInterfaceResponse delete_direct_link_interface(delete_direct_link_interface_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_direct_link_interface_request** | Option<[**DeleteDirectLinkInterfaceRequest**](DeleteDirectLinkInterfaceRequest.md)> |  |  |

### Return type

[**crate::models::DeleteDirectLinkInterfaceResponse**](DeleteDirectLinkInterfaceResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_direct_link_interfaces

> crate::models::ReadDirectLinkInterfacesResponse read_direct_link_interfaces(read_direct_link_interfaces_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_direct_link_interfaces_request** | Option<[**ReadDirectLinkInterfacesRequest**](ReadDirectLinkInterfacesRequest.md)> |  |  |

### Return type

[**crate::models::ReadDirectLinkInterfacesResponse**](ReadDirectLinkInterfacesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

