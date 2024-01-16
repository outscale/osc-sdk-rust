# CreateVpnConnectionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_gateway_id** | **String** | The ID of the client gateway. | 
**connection_type** | **String** | The type of VPN connection (only `ipsec.1` is supported). | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**static_routes_only** | Option<**bool**> | By default or if false, the VPN connection uses dynamic routing with Border Gateway Protocol (BGP). If true, routing is controlled using static routes. For more information about how to create and delete static routes, see [CreateVpnConnectionRoute](#createvpnconnectionroute) and [DeleteVpnConnectionRoute](#deletevpnconnectionroute). | [optional]
**virtual_gateway_id** | **String** | The ID of the virtual gateway. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


