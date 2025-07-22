# \VpnConnectionApi

All URIs are relative to *https://api.eu-west-2.outscale.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_vpn_connection**](VpnConnectionApi.md#create_vpn_connection) | **POST** /CreateVpnConnection | 
[**create_vpn_connection_route**](VpnConnectionApi.md#create_vpn_connection_route) | **POST** /CreateVpnConnectionRoute | 
[**delete_vpn_connection**](VpnConnectionApi.md#delete_vpn_connection) | **POST** /DeleteVpnConnection | 
[**delete_vpn_connection_route**](VpnConnectionApi.md#delete_vpn_connection_route) | **POST** /DeleteVpnConnectionRoute | 
[**read_vpn_connections**](VpnConnectionApi.md#read_vpn_connections) | **POST** /ReadVpnConnections | 
[**update_vpn_connection**](VpnConnectionApi.md#update_vpn_connection) | **POST** /UpdateVpnConnection | 



## create_vpn_connection

> crate::models::CreateVpnConnectionResponse create_vpn_connection(create_vpn_connection_request)


Creates a VPN connection between a specified virtual gateway and a specified client gateway.<br /> You can create only one VPN connection between a virtual gateway and a client gateway.<br /><br />  **[IMPORTANT]**<br /> This action can be done only if the virtual gateway is in the `available` state.<br /><br /> For more information, see [About VPN Connections](https://docs.outscale.com/en/userguide/About-VPN-Connections.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vpn_connection_request** | Option<[**CreateVpnConnectionRequest**](CreateVpnConnectionRequest.md)> |  |  |

### Return type

[**crate::models::CreateVpnConnectionResponse**](CreateVpnConnectionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vpn_connection_route

> crate::models::CreateVpnConnectionRouteResponse create_vpn_connection_route(create_vpn_connection_route_request)


Creates a static route to a VPN connection.<br /> This enables you to select the network flows sent by the virtual gateway to the target VPN connection.<br /><br /> For more information, see [About Routing Configuration for VPN Connections](https://docs.outscale.com/en/userguide/About-Routing-Configuration-for-VPN-Connections.html).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_vpn_connection_route_request** | Option<[**CreateVpnConnectionRouteRequest**](CreateVpnConnectionRouteRequest.md)> |  |  |

### Return type

[**crate::models::CreateVpnConnectionRouteResponse**](CreateVpnConnectionRouteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vpn_connection

> crate::models::DeleteVpnConnectionResponse delete_vpn_connection(delete_vpn_connection_request)


Deletes a specified VPN connection.<br /> If you want to delete a Net and all its dependencies, we recommend to detach the virtual gateway from the Net and delete the Net before deleting the VPN connection. This enables you to delete the Net without waiting for the VPN connection to be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_vpn_connection_request** | Option<[**DeleteVpnConnectionRequest**](DeleteVpnConnectionRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVpnConnectionResponse**](DeleteVpnConnectionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vpn_connection_route

> crate::models::DeleteVpnConnectionRouteResponse delete_vpn_connection_route(delete_vpn_connection_route_request)


Deletes a static route to a VPN connection previously created using the CreateVpnConnectionRoute method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_vpn_connection_route_request** | Option<[**DeleteVpnConnectionRouteRequest**](DeleteVpnConnectionRouteRequest.md)> |  |  |

### Return type

[**crate::models::DeleteVpnConnectionRouteResponse**](DeleteVpnConnectionRouteResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_vpn_connections

> crate::models::ReadVpnConnectionsResponse read_vpn_connections(read_vpn_connections_request)


Lists one or more VPN connections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_vpn_connections_request** | Option<[**ReadVpnConnectionsRequest**](ReadVpnConnectionsRequest.md)> |  |  |

### Return type

[**crate::models::ReadVpnConnectionsResponse**](ReadVpnConnectionsResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vpn_connection

> crate::models::UpdateVpnConnectionResponse update_vpn_connection(update_vpn_connection_request)


Modifies the specified attributes of a VPN connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_vpn_connection_request** | Option<[**UpdateVpnConnectionRequest**](UpdateVpnConnectionRequest.md)> |  |  |

### Return type

[**crate::models::UpdateVpnConnectionResponse**](UpdateVpnConnectionResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

