# Subnet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_ips_count** | Option<**i32**> | The number of available IPs in the Subnets. | [optional]
**ip_range** | Option<**String**> | The IP range in the Subnet, in CIDR notation (for example, 10.0.0.0/16). | [optional]
**map_public_ip_on_launch** | Option<**bool**> | If true, a public IP is assigned to the network interface cards (NICs) created in the specified Subnet. | [optional]
**net_id** | Option<**String**> | The ID of the Net in which the Subnet is. | [optional]
**state** | Option<**String**> | The state of the Subnet (`pending` \\| `available`). | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet. | [optional]
**subregion_name** | Option<**String**> | The name of the Subregion in which the Subnet is located. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the Subnet. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


