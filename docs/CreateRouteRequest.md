# CreateRouteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination_ip_range** | **String** | The IP range used for the destination match, in CIDR notation (for example, `10.0.0.0/24`). | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**gateway_id** | Option<**String**> | The ID of an internet service or virtual gateway attached to your Net. | [optional]
**nat_service_id** | Option<**String**> | The ID of a NAT service. | [optional]
**net_peering_id** | Option<**String**> | The ID of a Net peering. | [optional]
**nic_id** | Option<**String**> | The ID of a NIC. | [optional]
**route_table_id** | **String** | The ID of the route table for which you want to create a route. | 
**vm_id** | Option<**String**> | The ID of a NAT VM in your Net (attached to exactly one NIC). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


