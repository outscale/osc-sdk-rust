# LinkPrivateIpsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_relink** | Option<**bool**> | If true, allows an IP that is already assigned to another NIC in the same Subnet to be assigned to the NIC you specified. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**nic_id** | **String** | The ID of the NIC. | 
**private_ips** | Option<**Vec<String>**> | The secondary private IP or IPs you want to assign to the NIC within the IP range of the Subnet. | [optional]
**secondary_private_ip_count** | Option<**i32**> | The number of secondary private IPs to assign to the NIC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


