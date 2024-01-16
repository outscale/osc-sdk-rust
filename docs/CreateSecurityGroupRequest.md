# CreateSecurityGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | **String** | A description for the security group.<br /> This description can contain between 1 and 255 characters. Allowed characters are `a-z`, `A-Z`, `0-9`, accented letters, spaces, and `_.-:/()#,@[]+=&;{}!$*`. | 
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**net_id** | Option<**String**> | The ID of the Net for the security group. | [optional]
**security_group_name** | **String** | The name of the security group.<br /> This name must not start with `sg-`.<br /> This name must be unique and contain between 1 and 255 characters. Allowed characters are `a-z`, `A-Z`, `0-9`, spaces, and `_.-:/()#,@[]+=&;{}!$*`. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


