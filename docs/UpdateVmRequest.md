# UpdateVmRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions_on_next_boot** | Option<[**crate::models::ActionsOnNextBoot**](ActionsOnNextBoot.md)> |  | [optional]
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingVmUpdate>**](BlockDeviceMappingVmUpdate.md)> | One or more block device mappings of the VM. | [optional]
**bsu_optimized** | Option<**bool**> | This parameter is not available. It is present in our API for the sake of historical compatibility with AWS. | [optional]
**deletion_protection** | Option<**bool**> | If true, you cannot delete the VM unless you change this parameter back to false. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**is_source_dest_checked** | Option<**bool**> | (Net only) If true, the source/destination check is enabled. If false, it is disabled. | [optional]
**keypair_name** | Option<**String**> | The name of a keypair you want to associate with the VM.<br /> When you replace the keypair of a VM with another one, the metadata of the VM is modified to reflect the new public key, but the replacement is still not effective in the operating system of the VM. To complete the replacement and effectively apply the new keypair, you need to perform other actions inside the VM. For more information, see [Modifying the Keypair of a VM](https://docs.outscale.com/en/userguide/Modifying-the-Keypair-of-a-VM.html). | [optional]
**nested_virtualization** | Option<**bool**> | (dedicated tenancy only) If true, nested virtualization is enabled. If false, it is disabled. | [optional]
**performance** | Option<**String**> | The performance of the VM. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the VM. | [optional]
**user_data** | Option<**String**> | The Base64-encoded MIME user data, limited to 500 kibibytes (KiB). | [optional]
**vm_id** | **String** | The ID of the VM. | 
**vm_initiated_shutdown_behavior** | Option<**String**> | The VM behavior when you stop it. If set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is terminated. | [optional]
**vm_type** | Option<**String**> | The type of VM. For more information, see [VM Types](https://docs.outscale.com/en/userguide/VM-Types.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


