# FiltersNatService

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_tokens** | Option<**Vec<String>**> | The idempotency tokens provided when creating the NAT services. | [optional]
**nat_service_ids** | Option<**Vec<String>**> | The IDs of the NAT services. | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets in which the NAT services are. | [optional]
**states** | Option<**Vec<String>**> | The states of the NAT services (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**subnet_ids** | Option<**Vec<String>**> | The IDs of the Subnets in which the NAT services are. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the NAT services. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the NAT services. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the NAT services, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


