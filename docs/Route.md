# Route

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_method** | Option<**String**> | The method used to create the route. | [optional]
**destination_ip_range** | Option<**String**> | The IP range used for the destination match, in CIDR notation (for example, `10.0.0.0/24`). | [optional]
**destination_service_id** | Option<**String**> | The ID of the OUTSCALE service. | [optional]
**gateway_id** | Option<**String**> | The ID of the Internet service or virtual gateway attached to the Net. | [optional]
**nat_service_id** | Option<**String**> | The ID of a NAT service attached to the Net. | [optional]
**net_access_point_id** | Option<**String**> | The ID of the Net access point. | [optional]
**net_peering_id** | Option<**String**> | The ID of the Net peering. | [optional]
**nic_id** | Option<**String**> | The ID of the NIC. | [optional]
**state** | Option<**String**> | The state of a route in the route table (always `active`). | [optional]
**vm_account_id** | Option<**String**> | The account ID of the owner of the VM. | [optional]
**vm_id** | Option<**String**> | The ID of a VM specified in a route in the table. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


