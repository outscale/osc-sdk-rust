# LinkNic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delete_on_vm_deletion** | Option<**bool**> | If true, the NIC is deleted when the VM is terminated. | [optional]
**device_number** | Option<**i32**> | The device index for the NIC attachment (between 1 and 7, both included). | [optional]
**link_nic_id** | Option<**String**> | The ID of the NIC to attach. | [optional]
**state** | Option<**String**> | The state of the attachment (`attaching` \\| `attached` \\| `detaching` \\| `detached`). | [optional]
**vm_account_id** | Option<**String**> | The account ID of the owner of the VM. | [optional]
**vm_id** | Option<**String**> | The ID of the VM. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


