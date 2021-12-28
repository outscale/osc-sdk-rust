# UpdateNicRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A new description for the NIC. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**link_nic** | Option<[**crate::models::LinkNicToUpdate**](LinkNicToUpdate.md)> |  | [optional]
**nic_id** | **String** | The ID of the NIC you want to modify. | 
**security_group_ids** | Option<**Vec<String>**> | One or more IDs of security groups for the NIC.<br /> You must specify at least one group, even if you use the default security group in the Net. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


