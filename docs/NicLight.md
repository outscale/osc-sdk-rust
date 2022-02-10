# NicLight

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the owner of the NIC. | [optional]
**description** | Option<**String**> | The description of the NIC. | [optional]
**is_source_dest_checked** | Option<**bool**> | (Net only) If true, the source/destination check is enabled. If false, it is disabled. This value must be false for a NAT VM to perform network address translation (NAT) in a Net. | [optional]
**link_nic** | Option<[**crate::models::LinkNicLight**](LinkNicLight.md)> |  | [optional]
**link_public_ip** | Option<[**crate::models::LinkPublicIpLightForVm**](LinkPublicIpLightForVm.md)> |  | [optional]
**mac_address** | Option<**String**> | The Media Access Control (MAC) address of the NIC. | [optional]
**net_id** | Option<**String**> | The ID of the Net for the NIC. | [optional]
**nic_id** | Option<**String**> | The ID of the NIC. | [optional]
**private_dns_name** | Option<**String**> | The name of the private DNS. | [optional]
**private_ips** | Option<[**Vec<crate::models::PrivateIpLightForVm>**](PrivateIpLightForVm.md)> | The private IP or IPs of the NIC. | [optional]
**security_groups** | Option<[**Vec<crate::models::SecurityGroupLight>**](SecurityGroupLight.md)> | One or more IDs of security groups for the NIC. | [optional]
**state** | Option<**String**> | The state of the NIC (`available` \\| `attaching` \\| `in-use` \\| `detaching`). | [optional]
**subnet_id** | Option<**String**> | The ID of the Subnet for the NIC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


