# CreateVmsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingVmCreation>**](BlockDeviceMappingVmCreation.md)> | One or more block device mappings. | [optional]
**boot_on_creation** | Option<**bool**> | By default or if true, the VM is started on creation. If false, the VM is stopped on creation. | [optional]
**bsu_optimized** | Option<**bool**> | If true, the VM is created with optimized BSU I/O. | [optional]
**client_token** | Option<**String**> | A unique identifier which enables you to manage the idempotency. | [optional]
**deletion_protection** | Option<**bool**> | If true, you cannot terminate the VM using Cockpit, the CLI or the API. If false, you can. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**image_id** | **String** | The ID of the OMI used to create the VM. You can find the list of OMIs by calling the [ReadImages](#readimages) method. | 
**keypair_name** | Option<**String**> | The name of the keypair. | [optional]
**max_vms_count** | Option<**i32**> | The maximum number of VMs you want to create. If all the VMs cannot be created, the largest possible number of VMs above MinVmsCount is created. | [optional]
**min_vms_count** | Option<**i32**> | The minimum number of VMs you want to create. If this number of VMs cannot be created, no VMs are created. | [optional]
**nics** | Option<[**Vec<crate::models::NicForVmCreation>**](NicForVmCreation.md)> | One or more NICs. If you specify this parameter, you must define one NIC as the primary network interface of the VM with `0` as its device number. | [optional]
**performance** | Option<**String**> | The performance of the VM (`medium` \\| `high` \\|  `highest`). | [optional][default to Performance_High]
**placement** | Option<[**crate::models::Placement**](Placement.md)> |  | [optional]
**private_ips** | Option<**Vec<String>**> | One or more private IP addresses of the VM. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security group for the VMs. | [optional]
**security_groups** | Option<**Vec<String>**> | One or more names of security groups for the VMs. | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet in which you want to create the VM. | [optional]
**user_data** | Option<**String**> | Data or script used to add a specific configuration to the VM. It must be Base64-encoded and is limited to 500 kibibytes (KiB). | [optional]
**vm_initiated_shutdown_behavior** | Option<**String**> | The VM behavior when you stop it. By default or if set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is terminated. | [optional]
**vm_type** | Option<**String**> | The type of VM (`t2.small` by default).<br /> For more information, see [Instance Types](https://wiki.outscale.net/display/EN/Instance+Types). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


