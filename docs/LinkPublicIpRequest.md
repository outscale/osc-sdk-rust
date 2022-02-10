# LinkPublicIpRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_relink** | Option<**bool**> | If true, allows the public IP to be associated with the VM or NIC that you specify even if it is already associated with another VM or NIC. If false, prevents the EIP from being associated with the VM or NIC that you specify if it is already associated with another VM or NIC. (By default, true in the public Cloud, false in a Net.) | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**nic_id** | Option<**String**> | (Net only) The ID of the NIC. This parameter is required if the VM has more than one NIC attached. Otherwise, you need to specify the `VmId` parameter instead. You cannot specify both parameters at the same time. | [optional]
**private_ip** | Option<**String**> | (Net only) The primary or secondary private IP of the specified NIC. By default, the primary private IP. | [optional]
**public_ip** | Option<**String**> | The public IP. This parameter is required unless you use the `PublicIpId` parameter. | [optional]
**public_ip_id** | Option<**String**> | The allocation ID of the public IP. This parameter is required unless you use the `PublicIp` parameter. | [optional]
**vm_id** | Option<**String**> | The ID of the VM.<br /> - In the public Cloud, this parameter is required.<br /> - In a Net, this parameter is required if the VM has only one NIC. Otherwise, you need to specify the `NicId` parameter instead. You cannot specify both parameters at the same time. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


