# CreateApiAccessRuleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ca_ids** | Option<**Vec<String>**> |  One or more IDs of Client Certificate Authorities (CAs). | [optional]
**cns** | Option<**Vec<String>**> | One or more Client Certificate Common Names (CNs). If this parameter is specified, you must also specify the `CaIds` parameter. | [optional]
**description** | Option<**String**> | A description for the API access rule. | [optional]
**dry_run** | Option<**bool**> | If true, checks whether you have the required permissions to perform the action. | [optional]
**ip_ranges** | Option<**Vec<String>**> | One or more IP ranges, in CIDR notation (for example, `192.0.2.0/16`). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


