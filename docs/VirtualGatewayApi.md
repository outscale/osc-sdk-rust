# \VirtualGatewayApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_virtual_gateway**](VirtualGatewayApi.md#create_virtual_gateway) | **POST** /CreateVirtualGateway | 
[**delete_virtual_gateway**](VirtualGatewayApi.md#delete_virtual_gateway) | **POST** /DeleteVirtualGateway | 
[**link_virtual_gateway**](VirtualGatewayApi.md#link_virtual_gateway) | **POST** /LinkVirtualGateway | 
[**read_virtual_gateways**](VirtualGatewayApi.md#read_virtual_gateways) | **POST** /ReadVirtualGateways | 
[**unlink_virtual_gateway**](VirtualGatewayApi.md#unlink_virtual_gateway) | **POST** /UnlinkVirtualGateway | 
[**update_route_propagation**](VirtualGatewayApi.md#update_route_propagation) | **POST** /UpdateRoutePropagation | 



## create_virtual_gateway

> crate::models::CreateVirtualGatewayResponse create_virtual_gateway(create_virtual_gateway_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_virtual_gateway_request** | Option<[**CreateVirtualGatewayRequest**](CreateVirtualGatewayRequest.md)> |  |  |

### Return type

[**crate::models::CreateVirtualGatewayResponse**](CreateVirtualGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_virtual_gateway

> crate::models::DeleteVirtualGatewayResponse delete_virtual_gateway(delete_virtual_gateway_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_virtual_gateway_request** | Option<[**DeleteVirtualGatewayRequest**](DeleteVirtualGatewayRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVirtualGatewayResponse**](DeleteVirtualGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_virtual_gateway

> crate::models::LinkVirtualGatewayResponse link_virtual_gateway(link_virtual_gateway_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**link_virtual_gateway_request** | Option<[**LinkVirtualGatewayRequest**](LinkVirtualGatewayRequest.md)> |  |  |

### Return type

[**crate::models::LinkVirtualGatewayResponse**](LinkVirtualGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_virtual_gateways

> crate::models::ReadVirtualGatewaysResponse read_virtual_gateways(read_virtual_gateways_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_virtual_gateways_request** | Option<[**ReadVirtualGatewaysRequest**](ReadVirtualGatewaysRequest.md)> |  |  |

### Return type

[**crate::models::ReadVirtualGatewaysResponse**](ReadVirtualGatewaysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_virtual_gateway

> crate::models::UnlinkVirtualGatewayResponse unlink_virtual_gateway(unlink_virtual_gateway_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unlink_virtual_gateway_request** | Option<[**UnlinkVirtualGatewayRequest**](UnlinkVirtualGatewayRequest.md)> |  |  |

### Return type

[**crate::models::UnlinkVirtualGatewayResponse**](UnlinkVirtualGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_route_propagation

> crate::models::UpdateRoutePropagationResponse update_route_propagation(update_route_propagation_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_route_propagation_request** | Option<[**UpdateRoutePropagationRequest**](UpdateRoutePropagationRequest.md)> |  |  |

### Return type

[**crate::models::UpdateRoutePropagationResponse**](UpdateRoutePropagationResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

