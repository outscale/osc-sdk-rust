# DirectLinkInterface

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bgp_asn** | **i32** | The BGP (Border Gateway Protocol) ASN (Autonomous System Number) on the customer's side of the DirectLink interface. This number must be between `64512` and `65534`. | 
**bgp_key** | Option<**String**> | The BGP authentication key. | [optional]
**client_private_ip** | Option<**String**> | The IP on the customer's side of the DirectLink interface. | [optional]
**direct_link_interface_name** | **String** | The name of the DirectLink interface. | 
**outscale_private_ip** | Option<**String**> | The IP on the OUTSCALE side of the DirectLink interface. | [optional]
**virtual_gateway_id** | **String** | The ID of the target virtual gateway. | 
**vlan** | **i32** | The VLAN number associated with the DirectLink interface. This number must be unique and be between `2` and `4094`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


