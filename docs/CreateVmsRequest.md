# CreateVmsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingVmCreation>**](BlockDeviceMappingVmCreation.md)> | One or more block device mappings. | [optional]
**boot_on_creation** | Option<**bool**> | By default or if true, the VM is started on creation. If false, the VM is stopped on creation. | [optional][default to true]
**bsu_optimized** | Option<**bool**> | This parameter is not available. It is present in our API for the sake of historical compatibility with AWS. | [optional]
**client_token** | Option<**String**> | A unique identifier which enables you to manage the idempotency. | [optional]
**deletion_protection** | Option<**bool**> | If true, you cannot delete the VM unless you change this parameter back to false. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**image_id** | **String** | The ID of the OMI used to create the VM. You can find the list of OMIs by calling the [ReadImages](#readimages) method. | 
**keypair_name** | Option<**String**> | The name of the keypair. | [optional]
**max_vms_count** | Option<**i32**> | The maximum number of VMs you want to create. If all the VMs cannot be created, the largest possible number of VMs above MinVmsCount is created. | [optional]
**min_vms_count** | Option<**i32**> | The minimum number of VMs you want to create. If this number of VMs cannot be created, no VMs are created. | [optional]
**nested_virtualization** | Option<**bool**> | (dedicated tenancy only) If true, nested virtualization is enabled. If false, it is disabled. | [optional][default to false]
**nics** | Option<[**Vec<crate::models::NicForVmCreation>**](NicForVmCreation.md)> | One or more NICs. If you specify this parameter, you must not specify the `SubnetId` and `SubregionName` parameters. You also must define one NIC as the primary network interface of the VM with `0` as its device number. | [optional]
**performance** | Option<**String**> | The performance of the VM (`medium` \\| `high` \\|  `highest`). By default, `high`. This parameter is ignored if you specify a performance flag directly in the `VmType` parameter. | [optional][default to Performance_High]
**placement** | Option<[**crate::models::Placement**](Placement.md)> |  | [optional]
**private_ips** | Option<**Vec<String>**> | One or more private IPs of the VM. | [optional]
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security group for the VMs. | [optional]
**security_groups** | Option<**Vec<String>**> | One or more names of security groups for the VMs. | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet in which you want to create the VM. If you specify this parameter, you must not specify the `Nics` parameter. | [optional]
**user_data** | Option<**String**> | Data or script used to add a specific configuration to the VM. It must be Base64-encoded and is limited to 500 kibibytes (KiB). For more information about user data, see [Configuring a VM with User Data and OUTSCALE Tags](https://docs.outscale.com/en/userguide/Configuring-a-VM-with-User-Data-and-OUTSCALE-Tags.html). | [optional]
**vm_initiated_shutdown_behavior** | Option<**String**> | The VM behavior when you stop it. By default or if set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is terminated. | [optional][default to stop]
**vm_type** | Option<**String**> | The type of VM. You can specify a TINA type (in the `tinavW.cXrYpZ` or `tinavW.cXrY` format), or an AWS type (for example, `t2.small`, which is the default value).<br /> If you specify an AWS type, it is converted in the background to its corresponding TINA type, but the AWS type is still returned. If the specified or converted TINA type includes a performance flag, this performance flag is applied regardless of the value you may have provided in the `Performance` parameter. For more information, see [VM Types](https://docs.outscale.com/en/userguide/VM-Types.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


