# FiltersRouteTable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**link_route_table_ids** | Option<**Vec<String>**> | The IDs of the route tables involved in the associations. | [optional]
**link_route_table_link_route_table_ids** | Option<**Vec<String>**> | The IDs of the associations between the route tables and the Subnets. | [optional]
**link_route_table_main** | Option<**bool**> | If true, the route tables are the main ones for their Nets. | [optional]
**link_subnet_ids** | Option<**Vec<String>**> | The IDs of the Subnets involved in the associations. | [optional]
**net_ids** | Option<**Vec<String>**> | The IDs of the Nets for the route tables. | [optional]
**route_creation_methods** | Option<**Vec<String>**> | The methods used to create a route. | [optional]
**route_destination_ip_ranges** | Option<**Vec<String>**> | The IP ranges specified in routes in the tables. | [optional]
**route_destination_service_ids** | Option<**Vec<String>**> | The service IDs specified in routes in the tables. | [optional]
**route_gateway_ids** | Option<**Vec<String>**> | The IDs of the gateways specified in routes in the tables. | [optional]
**route_nat_service_ids** | Option<**Vec<String>**> | The IDs of the NAT services specified in routes in the tables. | [optional]
**route_net_peering_ids** | Option<**Vec<String>**> | The IDs of the Net peerings specified in routes in the tables. | [optional]
**route_states** | Option<**Vec<String>**> | The states of routes in the route tables (always `active`). | [optional]
**route_table_ids** | Option<**Vec<String>**> | The IDs of the route tables. | [optional]
**route_vm_ids** | Option<**Vec<String>**> | The IDs of the VMs specified in routes in the tables. | [optional]
**tag_keys** | Option<**Vec<String>**> | The keys of the tags associated with the route tables. | [optional]
**tag_values** | Option<**Vec<String>**> | The values of the tags associated with the route tables. | [optional]
**tags** | Option<**Vec<String>**> | The key/value combination of the tags associated with the route tables, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


