# \ClientGatewayApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client_gateway**](ClientGatewayApi.md#create_client_gateway) | **POST** /CreateClientGateway | 
[**delete_client_gateway**](ClientGatewayApi.md#delete_client_gateway) | **POST** /DeleteClientGateway | 
[**read_client_gateways**](ClientGatewayApi.md#read_client_gateways) | **POST** /ReadClientGateways | 



## create_client_gateway

> crate::models::CreateClientGatewayResponse create_client_gateway(create_client_gateway_request)


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

