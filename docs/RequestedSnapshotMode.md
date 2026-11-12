# RequestedSnapshotMode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | Option<**String**> | The requested mode (`application-consistent` \\| `crash-consistent`). | [optional]
**on_failure** | Option<**String**> | The fallback behavior if snapshot creation in `application-consistent` mode fails (`continue` \\| `abort`). By default or if set to `continue`, the snapshot is created in crash-consistent mode. If set to `abort`, the task is canceled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


