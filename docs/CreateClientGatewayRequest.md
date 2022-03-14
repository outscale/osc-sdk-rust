# CreateClientGatewayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asn** | **i32** | The Autonomous System Number (ASN) used by the Border Gateway Protocol (BGP) to find the path to your client gateway through the Internet. This number must be between 1 and 4294967295. | 
**connection_type** | **String** | The communication protocol used to establish tunnel with your client gateway (only `ipsec.1` is supported). | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**public_ip** | **String** | The public fixed IPv4 address of your client gateway. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


