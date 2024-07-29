# VmGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_date** | Option<**String**> | The date and time (UTC) at which the VM group was created. | [optional]
**description** | Option<**String**> | The description of the VM group. | [optional]
**positioning_strategy** | Option<**String**> | The positioning strategy of the VMs on hypervisors. By default, or if set to `no-strategy`, TINA determines the most adequate position for the VMs. If set to `attract`, the VMs are deployed on the same hypervisor, which improves network performance. If set to `repulse`, the VMs are deployed on a different hypervisor, which improves fault tolerance. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the VM group. | [optional]
**state** | Option<**String**> | The state of the VM group (`pending` \\| `available` \\| `scaling up` \\| `scaling down` \\| `deleting` \\| `deleted`). | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet for the VM group. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the VM. | [optional]
**vm_count** | Option<**i32**> | The number of VMs in the VM group. | [optional]
**vm_group_id** | Option<**String**> | The ID of the VM group. | [optional]
**vm_group_name** | Option<**String**> | The name of the VM group. | [optional]
**vm_ids** | Option<**Vec<String>**> | The IDs of the VMs in the VM group. | [optional]
**vm_template_id** | Option<**String**> | The ID of the VM template used by the VM group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


