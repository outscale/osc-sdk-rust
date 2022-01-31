# UpdateVmRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingVmUpdate>**](BlockDeviceMappingVmUpdate.md)> | One or more block device mappings of the VM. | [optional]
**bsu_optimized** | Option<**bool**> | If true, the VM is optimized for BSU I/O. | [optional]
**deletion_protection** | Option<**bool**> | If true, you cannot terminate the VM using Cockpit, the CLI or the API. If false, you can. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**is_source_dest_checked** | Option<**bool**> | (Net only) If true, the source/destination check is enabled. If false, it is disabled. This value must be false for a NAT VM to perform network address translation (NAT) in a Net. | [optional]
**keypair_name** | Option<**String**> | The name of the keypair.<br /> To complete the replacement, manually replace the old public key with the new public key in the ~/.ssh/authorized_keys file located in the VM. Restart the VM to apply the change. | [optional]
**performance** | Option<**String**> | The performance of the VM (`medium` \\| `high` \\|  `highest`). | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the VM. | [optional]
**user_data** | Option<**String**> | The Base64-encoded MIME user data, limited to 500 kibibytes (KiB). | [optional]
**vm_id** | **String** | The ID of the VM. | 
**vm_initiated_shutdown_behavior** | Option<**String**> | The VM behavior when you stop it. By default or if set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is terminated. | [optional]
**vm_type** | Option<**String**> | The type of VM. For more information, see [Instance Types](https://docs.outscale.com/en/userguide/Instance-Types.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


