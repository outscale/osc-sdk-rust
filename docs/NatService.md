# NatService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**nat_service_id** | Option<**String**> | The ID of the NAT service. | [optional]
**net_id** | Option<**String**> | The ID of the Net in which the NAT service is. | [optional]
**public_ips** | Option<[**Vec<crate::models::PublicIpLight>**](PublicIpLight.md)> | Information about the public IP or IPs associated with the NAT service. | [optional]
**state** | Option<**String**> | The state of the NAT service (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet in which the NAT service is. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the NAT service. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


