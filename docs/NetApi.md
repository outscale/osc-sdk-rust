# \NetApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_net**](NetApi.md#create_net) | **POST** /CreateNet | 
[**delete_net**](NetApi.md#delete_net) | **POST** /DeleteNet | 
[**read_nets**](NetApi.md#read_nets) | **POST** /ReadNets | 
[**update_net**](NetApi.md#update_net) | **POST** /UpdateNet | 



## create_net

> crate::models::CreateNetResponse create_net(create_net_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_net_request** | Option<[**CreateNetRequest**](CreateNetRequest.md)> |  |  |

### Return type

[**crate::models::CreateNetResponse**](CreateNetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_net

> crate::models::DeleteNetResponse delete_net(delete_net_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_net_request** | Option<[**DeleteNetRequest**](DeleteNetRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNetResponse**](DeleteNetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_nets

> crate::models::ReadNetsResponse read_nets(read_nets_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_nets_request** | Option<[**ReadNetsRequest**](ReadNetsRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetsResponse**](ReadNetsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_net

> crate::models::UpdateNetResponse update_net(update_net_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_net_request** | Option<[**UpdateNetRequest**](UpdateNetRequest.md)> |  |  |

### Return type

[**crate::models::UpdateNetResponse**](UpdateNetResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

