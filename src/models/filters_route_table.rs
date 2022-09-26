/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersRouteTable : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersRouteTable {
    /// The IDs of the route tables involved in the associations.
    #[serde(rename = "LinkRouteTableIds", skip_serializing_if = "Option::is_none")]
    pub link_route_table_ids: Option<Vec<String>>,
    /// The IDs of the associations between the route tables and the Subnets.
    #[serde(
        rename = "LinkRouteTableLinkRouteTableIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub link_route_table_link_route_table_ids: Option<Vec<String>>,
    /// If true, the route tables are the main ones for their Nets.
    #[serde(rename = "LinkRouteTableMain", skip_serializing_if = "Option::is_none")]
    pub link_route_table_main: Option<bool>,
    /// The IDs of the Subnets involved in the associations.
    #[serde(rename = "LinkSubnetIds", skip_serializing_if = "Option::is_none")]
    pub link_subnet_ids: Option<Vec<String>>,
    /// The IDs of the Nets for the route tables.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The methods used to create a route.
    #[serde(
        rename = "RouteCreationMethods",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_creation_methods: Option<Vec<String>>,
    /// The IP ranges specified in routes in the tables.
    #[serde(
        rename = "RouteDestinationIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_destination_ip_ranges: Option<Vec<String>>,
    /// The service IDs specified in routes in the tables.
    #[serde(
        rename = "RouteDestinationServiceIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_destination_service_ids: Option<Vec<String>>,
    /// The IDs of the gateways specified in routes in the tables.
    #[serde(rename = "RouteGatewayIds", skip_serializing_if = "Option::is_none")]
    pub route_gateway_ids: Option<Vec<String>>,
    /// The IDs of the NAT services specified in routes in the tables.
    #[serde(rename = "RouteNatServiceIds", skip_serializing_if = "Option::is_none")]
    pub route_nat_service_ids: Option<Vec<String>>,
    /// The IDs of the Net peering connections specified in routes in the tables.
    #[serde(rename = "RouteNetPeeringIds", skip_serializing_if = "Option::is_none")]
    pub route_net_peering_ids: Option<Vec<String>>,
    /// The states of routes in the route tables (always `active`).
    #[serde(rename = "RouteStates", skip_serializing_if = "Option::is_none")]
    pub route_states: Option<Vec<String>>,
    /// The IDs of the route tables.
    #[serde(rename = "RouteTableIds", skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    /// The IDs of the VMs specified in routes in the tables.
    #[serde(rename = "RouteVmIds", skip_serializing_if = "Option::is_none")]
    pub route_vm_ids: Option<Vec<String>>,
    /// The keys of the tags associated with the route tables.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the route tables.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the route tables, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersRouteTable {
    /// One or more filters.
    pub fn new() -> FiltersRouteTable {
        FiltersRouteTable {
            link_route_table_ids: None,
            link_route_table_link_route_table_ids: None,
            link_route_table_main: None,
            link_subnet_ids: None,
            net_ids: None,
            route_creation_methods: None,
            route_destination_ip_ranges: None,
            route_destination_service_ids: None,
            route_gateway_ids: None,
            route_nat_service_ids: None,
            route_net_peering_ids: None,
            route_states: None,
            route_table_ids: None,
            route_vm_ids: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
