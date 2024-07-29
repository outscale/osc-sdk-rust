# CreateClientGatewayRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asn** | **i32** | The Autonomous System Number (ASN) used by the Border Gateway Protocol (BGP) to find the path to your client gateway through the Internet. <br/> This number must be between `1` and `4294967295`. If you do not have an ASN, you can choose one between 64512 and 65534, or between 4200000000 and 4294967294. | 
**connection_type** | **String** | The communication protocol used to establish tunnel with your client gateway (always `ipsec.1`). | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**public_ip** | **String** | The public fixed IPv4 address of your client gateway. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


