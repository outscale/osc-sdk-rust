# CreateNicRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description for the NIC. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**private_ips** | Option<[**Vec<crate::models::PrivateIpLight>**](PrivateIpLight.md)> | The primary private IP address for the NIC.<br /> This IP address must be within the IP address range of the Subnet that you specify with the `SubnetId` attribute.<br /> If you do not specify this attribute, a random private IP address is selected within the IP address range of the Subnet. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the NIC. | [optional]
**subnet_id** | **String** | The ID of the Subnet in which you want to create the NIC. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


