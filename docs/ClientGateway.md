# ClientGateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asn** | Option<**i32**> | An Autonomous System Number (ASN) used by the Border Gateway Protocol (BGP) to find the path to your client gateway through the Internet. | [optional]
**client_gateway_id** | Option<**String**> | The ID of the client gateway. | [optional]
**connection_type** | Option<**String**> | The type of communication tunnel used by the client gateway (only `ipsec.1` is supported). | [optional]
**public_ip** | Option<**String**> | The public IPv4 address of the client gateway (must be a fixed address into a NATed network). | [optional]
**state** | Option<**String**> | The state of the client gateway (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the client gateway. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


