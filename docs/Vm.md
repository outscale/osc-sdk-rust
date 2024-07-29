# Vm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | The architecture of the VM (`i386` \\| `x86_64`). | [optional]
**block_device_mappings** | Option<[**Vec<crate::models::BlockDeviceMappingCreated>**](BlockDeviceMappingCreated.md)> | The block device mapping of the VM. | [optional]
**bsu_optimized** | Option<**bool**> | This parameter is not available. It is present in our API for the sake of historical compatibility with AWS. | [optional]
**client_token** | Option<**String**> | The idempotency token provided when launching the VM. | [optional]
**creation_date** | Option<**String**> | The date and time (UTC) at which the VM was created. | [optional]
**deletion_protection** | Option<**bool**> | If true, you cannot delete the VM unless you change this parameter back to false. | [optional]
**hypervisor** | Option<**String**> | The hypervisor type of the VMs (`ovm` \\| `xen`). | [optional]
**image_id** | Option<**String**> | The ID of the OMI used to create the VM. | [optional]
**is_source_dest_checked** | Option<**bool**> | (Net only) If true, the source/destination check is enabled. If false, it is disabled. | [optional]
**keypair_name** | Option<**String**> | The name of the keypair used when launching the VM. | [optional]
**launch_number** | Option<**i32**> | The number for the VM when launching a group of several VMs (for example, `0`, `1`, `2`, and so on). | [optional]
**nested_virtualization** | Option<**bool**> | If true, nested virtualization is enabled. If false, it is disabled. | [optional]
**net_id** | Option<**String**> | The ID of the Net in which the VM is running. | [optional]
**nics** | Option<[**Vec<crate::models::NicLight>**](NicLight.md)> | (Net only) The network interface cards (NICs) the VMs are attached to. | [optional]
**os_family** | Option<**String**> | Indicates the operating system (OS) of the VM. | [optional]
**performance** | Option<**String**> | The performance of the VM (`medium` \\| `high` \\|  `highest`). | [optional]
**placement** | Option<[**crate::models::Placement**](Placement.md)> |  | [optional]
**private_dns_name** | Option<**String**> | The name of the private DNS. | [optional]
**private_ip** | Option<**String**> | The primary private IP of the VM. | [optional]
**product_codes** | Option<**Vec<String>**> | The product codes associated with the OMI used to create the VM. | [optional]
**public_dns_name** | Option<**String**> | The name of the public DNS. | [optional]
**public_ip** | Option<**String**> | The public IP of the VM. | [optional]
**reservation_id** | Option<**String**> | The reservation ID of the VM. | [optional]
**root_device_name** | Option<**String**> | The name of the root device for the VM (for example, `/dev/sda1`). | [optional]
**root_device_type** | Option<**String**> | The type of root device used by the VM (always `bsu`). | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroupLight>**](SecurityGroupLight.md)> | One or more security groups associated with the VM. | [optional]
**state** | Option<**String**> | The state of the VM (`pending` \\| `running` \\| `stopping` \\| `stopped` \\| `shutting-down` \\| `terminated` \\| `quarantine`). | [optional]
**state_reason** | Option<**String**> | The reason explaining the current state of the VM. | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet for the VM. | [optional]
**tags** | Option<[**Vec<crate::models::ResourceTag>**](ResourceTag.md)> | One or more tags associated with the VM. | [optional]
**user_data** | Option<**String**> | The Base64-encoded MIME user data. | [optional]
**vm_id** | Option<**String**> | The ID of the VM. | [optional]
**vm_initiated_shutdown_behavior** | Option<**String**> | The VM behavior when you stop it. If set to `stop`, the VM stops. If set to `restart`, the VM stops then automatically restarts. If set to `terminate`, the VM stops and is deleted. | [optional]
**vm_type** | Option<**String**> | The type of VM. For more information, see [VM Types](https://docs.outscale.com/en/userguide/VM-Types.html). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


