# FiltersVpnConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asns** | Option<**Vec<i32>**> | The Border Gateway Protocol (BGP) Autonomous System Numbers (ASNs) of the connections. | [optional]
**client_gateway_ids** | Option<**Vec<String>**> | The IDs of the client gateways. | [optional]
**connection_types** | Option<**Vec<String>**> | The types of the VPN connections (only `ipsec.1` is supported). | [optional]
**route_destination_ip_ranges** | Option<**Vec<String>**> | The destination IP ranges. | [optional]
**states** | Option<**Vec<String>**> | The states of the VPN connections (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**static_routes_only** | Option<**bool**> | If false, the VPN connection uses dynamic routing with Border Gateway Protocol (BGP). If true, routing is controlled using static routes. For more information about how to create and delete static routes, see [CreateVpnConnectionRoute](#createvpnconnectionroute) and [DeleteVpnConnectionRoute](#deletevpnconnectionroute). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the VPN connections. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the VPN connections. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the VPN connections, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]
**virtual_gateway_ids** | Option<**Vec<String>**> | The IDs of the virtual gateways. | [optional]
**vpn_connection_ids** | Option<**Vec<String>**> | The IDs of the VPN connections. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


