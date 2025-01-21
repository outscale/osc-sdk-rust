# CreateNetPeeringRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepter_net_id** | **String** | The ID of the Net you want to connect with. <br/ > <br/ >  If the Net does not belong to you, you must also specify the `AccepterOwnerId` parameter with the account ID owning the Net you want to connect with. | 
**accepter_owner_id** | Option<**String**> | The account ID of the owner of the Net you want to connect with. By default, the account ID of the owner of the Net from which the peering request is sent. <br /><br/ >  This parameter is required if the Net you want to connect with does not belong to you. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**source_net_id** | **String** | The ID of the Net you send the peering request from. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


