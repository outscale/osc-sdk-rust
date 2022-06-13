# NicForVmCreation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | If true, the NIC is deleted when the VM is terminated. You can specify this parameter only for a new NIC. To modify this value for an existing NIC, see [UpdateNic](#updatenic). | [optional]
**description** | Option<**String**> | The description of the NIC, if you are creating a NIC when creating the VM. | [optional]
**device_number** | Option<**i32**> | The index of the VM device for the NIC attachment (between `0` and `7`, both included). This parameter is required if you create a NIC when creating the VM. | [optional]
**nic_id** | Option<**String**> | The ID of the NIC, if you are attaching an existing NIC when creating a VM. | [optional]
**private_ips** | Option<[**Vec<crate::models::PrivateIpLight>**](PrivateIpLight.md)> | One or more private IPs to assign to the NIC, if you create a NIC when creating a VM. Only one private IP can be the primary private IP. | [optional]
**secondary_private_ip_count** | Option<**i32**> | The number of secondary private IPs, if you create a NIC when creating a VM. This parameter cannot be specified if you specified more than one private IP in the `PrivateIps` parameter. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the NIC, if you create a NIC when creating a VM. | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet for the NIC, if you create a NIC when creating a VM. This parameter is required if you create a NIC when creating the VM. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


