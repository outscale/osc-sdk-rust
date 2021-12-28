# FiltersClientGateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asns** | Option<**Vec<i32>**> | The Border Gateway Protocol (BGP) Autonomous System Numbers (ASNs) of the connections. | [optional]
**client_gateway_ids** | Option<**Vec<String>**> | The IDs of the client gateways. | [optional]
**connection_types** | Option<**Vec<String>**> | The types of communication tunnels used by the client gateways (only `ipsec.1` is supported). | [optional]
**public_ips** | Option<**Vec<String>**> | The public IPv4 addresses of the client gateways. | [optional]
**states** | Option<**Vec<String>**> | The states of the client gateways (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the client gateways. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the client gateways. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the client gateways, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


