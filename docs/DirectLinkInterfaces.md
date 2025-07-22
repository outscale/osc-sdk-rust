# DirectLinkInterfaces

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the owner of the DirectLink interface. | [optional]
**bgp_asn** | Option<**i32**> | The BGP (Border Gateway Protocol) ASN (Autonomous System Number) on the customer's side of the DirectLink interface. | [optional]
**bgp_key** | Option<**String**> | The BGP authentication key. | [optional]
**client_private_ip** | Option<**String**> | The IP on the customer's side of the DirectLink interface. | [optional]
**direct_link_id** | Option<**String**> | The ID of the DirectLink. | [optional]
**direct_link_interface_id** | Option<**String**> | The ID of the DirectLink interface. | [optional]
**direct_link_interface_name** | Option<**String**> | The name of the DirectLink interface. | [optional]
**interface_type** | Option<**String**> | The type of the DirectLink interface (always `private`). | [optional]
**location** | Option<**String**> | The datacenter where the DirectLink interface is located. | [optional]
**mtu** | Option<**i32**> | The maximum transmission unit (MTU) of the DirectLink interface, in bytes. | [optional]
**outscale_private_ip** | Option<**String**> | The IP on the OUTSCALE side of the DirectLink interface. | [optional]
**state** | Option<**String**> | The state of the DirectLink interface (`pending` \\| `available` \\| `deleting` \\| `deleted` \\| `confirming` \\| `rejected` \\| `expired`). | [optional]
**virtual_gateway_id** | Option<**String**> | The ID of the target virtual gateway. | [optional]
**vlan** | Option<**i32**> | The VLAN number associated with the DirectLink interface. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


