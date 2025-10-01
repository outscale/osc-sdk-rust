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


Accepts a Net peering request.<br /> To accept this request, you must be the owner of the peer Net. If you do not accept the request within 7 days, the state of the Net peering becomes `expired`.<br /><br />  **[NOTE]**<br /> A peering connection between two Nets works both ways. Therefore, when an A-to-B peering connection is accepted, any pending B-to-A peering connection is automatically rejected as redundant.

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


Requests a Net peering between a Net you own and a peer Net that belongs to you or another account.<br /> This action creates a Net peering that remains in the `pending-acceptance` state until it is accepted by the owner of the peer Net. If the owner of the peer Net does not accept the request within 7 days, the state of the Net peering becomes `expired`. For more information, see [AcceptNetPeering](#acceptnetpeering).<br /><br />  **[IMPORTANT]**<br /> * The two Nets must not have overlapping IP ranges. Otherwise, the Net peering is in the `failed` state.<br /> * A peering connection between two Nets works both ways. If an A-to-B connection is already created and accepted, creating a B-to-A connection is not necessary and would be automatically rejected.  For more information, see [About Net Peerings](https://docs.outscale.com/en/userguide/About-Net-Peerings.html).

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


Deletes a Net peering.<br /> If the Net peering is in the `active` state, it can be deleted either by the owner of the requester Net or the owner of the peer Net.<br /> If it is in the `pending-acceptance` state, it can be deleted only by the owner of the requester Net.<br /> If it is in the `rejected`, `failed`, or `expired` states, it cannot be deleted.

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


Lists one or more peering connections between two Nets.

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


Rejects a Net peering request.<br /> The Net peering must be in the `pending-acceptance` state to be rejected. The rejected Net peering is then in the `rejected` state.

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

