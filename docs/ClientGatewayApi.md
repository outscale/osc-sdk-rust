# \ClientGatewayApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client_gateway**](ClientGatewayApi.md#create_client_gateway) | **POST** /CreateClientGateway | 
[**delete_client_gateway**](ClientGatewayApi.md#delete_client_gateway) | **POST** /DeleteClientGateway | 
[**read_client_gateways**](ClientGatewayApi.md#read_client_gateways) | **POST** /ReadClientGateways | 



## create_client_gateway

> crate::models::CreateClientGatewayResponse create_client_gateway(create_client_gateway_request)


Provides information about your client gateway.<br /> This action registers information to identify the client gateway that you deployed in your network.<br /> To open a tunnel to the client gateway, you must provide the communication protocol type, the fixed public IP of the gateway, and an Autonomous System Number (ASN).<br /><br /> For more information, see [About Client Gateways](https://docs.outscale.com/en/userguide/About-Client-Gateways.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_client_gateway_request** | Option<[**CreateClientGatewayRequest**](CreateClientGatewayRequest.md)> |  |  |

### Return type

[**crate::models::CreateClientGatewayResponse**](CreateClientGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client_gateway

> crate::models::DeleteClientGatewayResponse delete_client_gateway(delete_client_gateway_request)


Deletes a client gateway.<br /> You must delete the VPN connection before deleting the client gateway.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_client_gateway_request** | Option<[**DeleteClientGatewayRequest**](DeleteClientGatewayRequest.md)> |  |  |

### Return type

[**crate::models::DeleteClientGatewayResponse**](DeleteClientGatewayResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_client_gateways

> crate::models::ReadClientGatewaysResponse read_client_gateways(read_client_gateways_request)


Lists one or more of your client gateways.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_client_gateways_request** | Option<[**ReadClientGatewaysRequest**](ReadClientGatewaysRequest.md)> |  |  |

### Return type

[**crate::models::ReadClientGatewaysResponse**](ReadClientGatewaysResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

