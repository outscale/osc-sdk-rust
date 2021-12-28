# FiltersSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_aliases** | Option<**Vec<String>**> | The account aliases of the owners of the snapshots. | [optional]
**account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the snapshots. | [optional]
**descriptions** | Option<**Vec<String>**> | The descriptions of the snapshots. | [optional]
**permissions_to_create_volume_account_ids** | Option<**Vec<String>**> | The account IDs of one or more users who have permissions to create volumes. | [optional]
**permissions_to_create_volume_global_permission** | Option<**bool**> | If true, lists all public volumes. If false, lists all private volumes. | [optional]
**progresses** | Option<**Vec<i32>**> | The progresses of the snapshots, as a percentage. | [optional]
**snapshot_ids** | Option<**Vec<String>**> | The IDs of the snapshots. | [optional]
**states** | Option<**Vec<String>**> | The states of the snapshots (`in-queue` \\| `completed` \\| `error`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the snapshots. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the snapshots. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the snapshots, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]
**volume_ids** | Option<**Vec<String>**> | The IDs of the volumes used to create the snapshots. | [optional]
**volume_sizes** | Option<**Vec<i32>**> | The sizes of the volumes used to create the snapshots, in gibibytes (GiB). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


