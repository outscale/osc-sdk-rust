# PermissionsOnResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_ids** | Option<**Vec<String>**> | One or more account IDs that the permission is associated with. | [optional]
**global_permission** | Option<**bool**> | A global permission for all accounts.<br /> (Request) Set this parameter to true to make the resource public (if the parent parameter is `Additions`) or to make the resource private (if the parent parameter is `Removals`).<br /> (Response) If true, the resource is public. If false, the resource is private. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


