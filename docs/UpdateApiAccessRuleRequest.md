# UpdateApiAccessRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_access_rule_id** | **String** | The ID of the API access rule you want to update. | 
**ca_ids** | Option<**Vec<String>**> | One or more IDs of Client Certificate Authorities (CAs). | [optional]
**cns** | Option<**Vec<String>**> | One or more Client Certificate Common Names (CNs). | [optional]
**description** | Option<**String**> | A new description for the API access rule. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**ip_ranges** | Option<**Vec<String>**> | One or more IP ranges, in CIDR notation (for example, 192.0.2.0/16). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


