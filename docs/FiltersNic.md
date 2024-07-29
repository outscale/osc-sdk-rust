# FiltersNic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**descriptions** | Option<**Vec<String>**> | The descriptions of the NICs. | [optional]
**is_source_dest_check** | Option<**bool**> | Whether the source/destination checking is enabled (true) or disabled (false). | [optional]
**link_nic_delete_on_vm_deletion** | Option<**bool**> | Whether the NICs are deleted when the VMs they are attached to are terminated. | [optional]
**link_nic_device_numbers** | Option<**Vec<i32>**> | The device numbers the NICs are attached to. | [optional]
**link_nic_link_nic_ids** | Option<**Vec<String>**> | The attachment IDs of the NICs. | [optional]
**link_nic_states** | Option<**Vec<String>**> | The states of the attachments. | [optional]
**link_nic_vm_account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the VMs the NICs are attached to. | [optional]
**link_nic_vm_ids** | Option<**Vec<String>**> | The IDs of the VMs the NICs are attached to. | [optional]
**link_public_ip_account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the public IPs associated with the NICs. | [optional]
**link_public_ip_link_public_ip_ids** | Option<**Vec<String>**> | The association IDs returned when the public IPs were associated with the NICs. | [optional]
**link_public_ip_public_dns_names** | Option<**Vec<String>**> | The public DNS names associated with the public IPs. | [optional]
**link_public_ip_public_ip_ids** | Option<**Vec<String>**> | The allocation IDs returned when the public IPs were allocated to their accounts. | [optional]
**link_public_ip_public_ips** | Option<**Vec<String>**> | The public IPs associated with the NICs. | [optional]
**mac_addresses** | Option<**Vec<String>**> | The Media Access Control (MAC) addresses of the NICs. | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets where the NICs are located. | [optional]
**nic_ids** | Option<**Vec<String>**> | The IDs of the NICs. | [optional]
**private_dns_names** | Option<**Vec<String>**> | The private DNS names associated with the primary private IPs. | [optional]
**private_ips_link_public_ip_account_ids** | Option<**Vec<String>**> | The account IDs of the owner of the public IPs associated with the private IPs. | [optional]
**private_ips_link_public_ip_public_ips** | Option<**Vec<String>**> | The public IPs associated with the private IPs. | [optional]
**private_ips_primary_ip** | Option<**bool**> | Whether the private IP is the primary IP associated with the NIC. | [optional]
**private_ips_private_ips** | Option<**Vec<String>**> | The private IPs of the NICs. | [optional]
**security_group_ids** | Option<**Vec<String>**> | The IDs of the security groups associated with the NICs. | [optional]
**security_group_names** | Option<**Vec<String>**> | The names of the security groups associated with the NICs. | [optional]
**states** | Option<**Vec<String>**> | The states of the NICs. | [optional]
**subnet_ids** | Option<**Vec<String>**> | The IDs of the Subnets for the NICs. | [optional]
**subregion_names** | Option<**Vec<String>**> | The Subregions where the NICs are located. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the NICs. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the NICs. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the NICs, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


