# ListenerRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> | The type of action for the rule (always `forward`). | [optional]
**host_name_pattern** | Option<**String**> | A host-name pattern for the rule, with a maximum length of 128 characters. This host-name pattern supports maximum three wildcards, and must not contain any special characters except `-.?`. | [optional]
**listener_id** | Option<**i32**> | The ID of the listener. | [optional]
**listener_rule_id** | Option<**i32**> | The ID of the listener rule. | [optional]
**listener_rule_name** | Option<**String**> | A human-readable name for the listener rule. | [optional]
**path_pattern** | Option<**String**> | A path pattern for the rule, with a maximum length of 128 characters. This path pattern supports maximum three wildcards, and must not contain any special characters except `_-.$/~&quot;'@:+?`. | [optional]
**priority** | Option<**i32**> | The priority level of the listener rule, between `1` and `19999` both included. Each rule must have a unique priority level. Otherwise, an error is returned. | [optional]
**vm_ids** | Option<**Vec<String>**> | The IDs of the backend VMs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


