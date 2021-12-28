# \NetPeeringApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_net_peering**](NetPeeringApi.md#accept_net_peering) | **POST** /AcceptNetPeering | 
[**create_net_peering**](NetPeeringApi.md#create_net_peering) | **POST** /CreateNetPeering | 
[**delete_net_peering**](NetPeeringApi.md#delete_net_peering) | **POST** /DeleteNetPeering | 
[**read_net_peerings**](NetPeeringApi.md#read_net_peerings) | **POST** /ReadNetPeerings | 
[**reject_net_peering**](NetPeeringApi.md#reject_net_peering) | **POST** /RejectNetPeering | 



## accept_net_peering

> crate::models::AcceptNetPeeringResponse accept_net_peering(accept_net_peering_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_net_peering_request** | Option<[**AcceptNetPeeringRequest**](AcceptNetPeeringRequest.md)> |  |  |

### Return type

[**crate::models::AcceptNetPeeringResponse**](AcceptNetPeeringResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_net_peering

> crate::models::CreateNetPeeringResponse create_net_peering(create_net_peering_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_net_peering_request** | Option<[**CreateNetPeeringRequest**](CreateNetPeeringRequest.md)> |  |  |

### Return type

[**crate::models::CreateNetPeeringResponse**](CreateNetPeeringResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_net_peering

> crate::models::DeleteNetPeeringResponse delete_net_peering(delete_net_peering_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_net_peering_request** | Option<[**DeleteNetPeeringRequest**](DeleteNetPeeringRequest.md)> |  |  |

### Return type

[**crate::models::DeleteNetPeeringResponse**](DeleteNetPeeringResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_net_peerings

> crate::models::ReadNetPeeringsResponse read_net_peerings(read_net_peerings_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_net_peerings_request** | Option<[**ReadNetPeeringsRequest**](ReadNetPeeringsRequest.md)> |  |  |

### Return type

[**crate::models::ReadNetPeeringsResponse**](ReadNetPeeringsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_net_peering

> crate::models::RejectNetPeeringResponse reject_net_peering(reject_net_peering_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reject_net_peering_request** | Option<[**RejectNetPeeringRequest**](RejectNetPeeringRequest.md)> |  |  |

### Return type

[**crate::models::RejectNetPeeringResponse**](RejectNetPeeringResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

