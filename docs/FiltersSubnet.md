# FiltersSubnet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_ips_counts** | Option<**Vec<i32>**> | The number of available IPs. | [optional]
**ip_ranges** | Option<**Vec<String>**> | The IP ranges in the Subnets, in CIDR notation (for example, `10.0.0.0/16`). | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets in which the Subnets are. | [optional]
**states** | Option<**Vec<String>**> | The states of the Subnets (`pending` \\| `available`). | [optional]
**subnet_ids** | Option<**Vec<String>**> | The IDs of the Subnets. | [optional]
**subregion_names** | Option<**Vec<String>**> | The names of the Subregions in which the Subnets are located. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the Subnets. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the Subnets. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the Subnets, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


