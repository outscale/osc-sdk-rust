# BackendVmHealth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the state of the back-end VM. | [optional]
**state** | Option<**String**> | The state of the back-end VM (`InService` \\| `OutOfService` \\| `Unknown`). | [optional]
**state_reason** | Option<**String**> | Information about the cause of `OutOfService` VMs.<br /> Specifically, whether the cause is Elastic Load Balancing or the VM (`ELB` \\| `Instance` \\| `N/A`). | [optional]
**vm_id** | Option<**String**> | The ID of the back-end VM. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


