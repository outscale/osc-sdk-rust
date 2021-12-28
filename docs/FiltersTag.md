# FiltersTag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**keys** | Option<**Vec<String>**> | The keys of the tags that are assigned to the resources. You can use this filter alongside the `Values` filter. In that case, you filter the resources corresponding to each tag, regardless of the other filter. | [optional]
**resource_ids** | Option<**Vec<String>**> | The IDs of the resources with which the tags are associated. | [optional]
**resource_types** | Option<**Vec<String>**> | The resource type (`vm` \\| `image` \\| `volume` \\| `snapshot` \\| `public-ip` \\| `security-group` \\| `route-table` \\| `nic` \\| `net` \\| `subnet` \\| `net-peering` \\| `net-access-point` \\| `nat-service` \\| `internet-service` \\| `client-gateway` \\| `virtual-gateway` \\| `vpn-connection` \\| `dhcp-options` \\| `task`). | [optional]
**values** | Option<**Vec<String>**> | The values of the tags that are assigned to the resources. You can use this filter alongside the `TagKeys` filter. In that case, you filter the resources corresponding to each tag, regardless of the other filter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


