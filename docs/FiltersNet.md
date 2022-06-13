# FiltersNet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dhcp_options_set_ids** | Option<**Vec<String>**> | The IDs of the DHCP options sets. | [optional]
**ip_ranges** | Option<**Vec<String>**> | The IP ranges for the Nets, in CIDR notation (for example, `10.0.0.0/16`). | [optional]
**is_default** | Option<**bool**> | If true, the Net used is the default one. | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets. | [optional]
**states** | Option<**Vec<String>**> | The states of the Nets (`pending` \\| `available`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the Nets. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the Nets. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the Nets, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


