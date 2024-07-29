# FiltersVirtualGateway

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_types** | Option<**Vec<String>**> | The types of the virtual gateways (always `ipsec.1`). | [optional]
**link_net_ids** | Option<**Vec<String>**> | The IDs of the Nets the virtual gateways are attached to. | [optional]
**link_states** | Option<**Vec<String>**> | The current states of the attachments between the virtual gateways and the Nets (`attaching` \\| `attached` \\| `detaching` \\| `detached`). | [optional]
**states** | Option<**Vec<String>**> | The states of the virtual gateways (`pending` \\| `available` \\| `deleting` \\| `deleted`). | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the virtual gateways. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the virtual gateways. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the virtual gateways, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]
**virtual_gateway_ids** | Option<**Vec<String>**> | The IDs of the virtual gateways. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


