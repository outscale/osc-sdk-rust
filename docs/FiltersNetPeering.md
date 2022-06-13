# FiltersNetPeering

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepter_net_account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the peer Nets. | [optional]
**accepter_net_ip_ranges** | Option<**Vec<String>**> | The IP ranges of the peer Nets, in CIDR notation (for example, `10.0.0.0/24`). | [optional]
**accepter_net_net_ids** | Option<**Vec<String>**> | The IDs of the peer Nets. | [optional]
**net_peering_ids** | Option<**Vec<String>**> | The IDs of the Net peering connections. | [optional]
**source_net_account_ids** | Option<**Vec<String>**> | The account IDs of the owners of the peer Nets. | [optional]
**source_net_ip_ranges** | Option<**Vec<String>**> | The IP ranges of the peer Nets. | [optional]
**source_net_net_ids** | Option<**Vec<String>**> | The IDs of the peer Nets. | [optional]
**state_messages** | Option<**Vec<String>**> | Additional information about the states of the Net peering connections. | [optional]
**state_names** | Option<**Vec<String>**> | The states of the Net peering connections (`pending-acceptance` \\| `active` \\| `rejected` \\| `failed` \\| `expired` \\| `deleted`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the Net peering connections. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the Net peering connections. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the Net peering connections, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


