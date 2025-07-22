# CreateVmGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description for the VM group. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**positioning_strategy** | Option<**String**> | The positioning strategy of VMs on hypervisors. If set to `no-strategy`, our orchestrator determines the most adequate position for your VMs. If set to `attract`, your VMs are deployed on the same hypervisor, which improves network performance. If set to `repulse`, your VMs are deployed on a different hypervisor, which improves fault tolerance. | [optional][default to PositioningStrategy_NoStrategy]
**security_group_ids** | **Vec<String>** | One or more IDs of security groups for the VM group. | 
**subnet_id** | **String** | The ID of the Subnet in which you want to create the VM group. | 
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags to add to the VM group. | [optional]
**vm_count** | **i32** | The number of VMs deployed in the VM group. | 
**vm_group_name** | **String** | The name of the VM group. | 
**vm_template_id** | **String** | The ID of the VM template used to launch VMs in the VM group. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


