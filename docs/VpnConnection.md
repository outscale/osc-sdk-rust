# VpnConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_gateway_configuration** | Option<**String**> | Example configuration for the client gateway. | [optional]
**client_gateway_id** | Option<**String**> | The ID of the client gateway used on the client end of the connection. | [optional]
**connection_type** | Option<**String**> | The type of VPN connection (always `ipsec.1`). | [optional]
**routes** | Option<[**Vec<crate::models::RouteLight>**](RouteLight.md)> | Information about one or more static routes associated with the VPN connection, if any. | [optional]
**state** | Option<**String**> | The state of the VPN connection (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**static_routes_only** | Option<**bool**> | If false, the VPN connection uses dynamic routing with Border Gateway Protocol (BGP). If true, routing is controlled using static routes. For more information about how to create and delete static routes, see [CreateVpnConnectionRoute](#createvpnconnectionroute) and [DeleteVpnConnectionRoute](#deletevpnconnectionroute). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the VPN connection. | [optional]
**virtual_gateway_id** | Option<**String**> | The ID of the virtual gateway used on the OUTSCALE end of the connection. | [optional]
**vpn_connection_id** | Option<**String**> | The ID of the VPN connection. | [optional]
**vpn_options** | Option<[**crate::models::VpnOptions**](VpnOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


