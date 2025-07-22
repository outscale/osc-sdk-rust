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


Creates a virtual gateway.<br /> A virtual gateway is the access point on the Net side of a VPN connection.<br /><br /> For more information, see [About Virtual Gateways](https://docs.outscale.com/en/userguide/About-Virtual-Gateways.html).

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


Deletes a specified virtual gateway.<br /> Before deleting a virtual gateway, we recommend to detach it from the Net and delete the VPN connection.

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


Attaches a virtual gateway to a Net.  **[IMPORTANT]**<br /> This action can be done only if the virtual gateway is in the `available` state.

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


Lists one or more virtual gateways.

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


Detaches a virtual gateway from a Net.<br /> You must wait until the virtual gateway is in the detached state before you can attach another Net to it or delete the Net it was previously attached to.

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


Configures the propagation of routes to a specified route table of a Net by a virtual gateway.

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

