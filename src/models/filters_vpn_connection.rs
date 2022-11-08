/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVpnConnection : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVpnConnection {
    /// The Border Gateway Protocol (BGP) Autonomous System Numbers (ASNs) of the connections.
    #[serde(rename = "BgpAsns", skip_serializing_if = "Option::is_none")]
    pub bgp_asns: Option<Vec<i32>>,
    /// The IDs of the client gateways.
    #[serde(rename = "ClientGatewayIds", skip_serializing_if = "Option::is_none")]
    pub client_gateway_ids: Option<Vec<String>>,
    /// The types of the VPN connections (only `ipsec.1` is supported).
    #[serde(rename = "ConnectionTypes", skip_serializing_if = "Option::is_none")]
    pub connection_types: Option<Vec<String>>,
    /// The destination IP ranges.
    #[serde(
        rename = "RouteDestinationIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub route_destination_ip_ranges: Option<Vec<String>>,
    /// The states of the VPN connections (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// If false, the VPN connection uses dynamic routing with Border Gateway Protocol (BGP). If true, routing is controlled using static routes. For more information about how to create and delete static routes, see [CreateVpnConnectionRoute](#createvpnconnectionroute) and [DeleteVpnConnectionRoute](#deletevpnconnectionroute).
    #[serde(rename = "StaticRoutesOnly", skip_serializing_if = "Option::is_none")]
    pub static_routes_only: Option<bool>,
    /// The keys of the tags associated with the VPN connections.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the VPN connections.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the VPN connections, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The IDs of the virtual gateways.
    #[serde(rename = "VirtualGatewayIds", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_ids: Option<Vec<String>>,
    /// The IDs of the VPN connections.
    #[serde(rename = "VpnConnectionIds", skip_serializing_if = "Option::is_none")]
    pub vpn_connection_ids: Option<Vec<String>>,
}

impl FiltersVpnConnection {
    /// One or more filters.
    pub fn new() -> FiltersVpnConnection {
        FiltersVpnConnection {
            bgp_asns: None,
            client_gateway_ids: None,
            connection_types: None,
            route_destination_ip_ranges: None,
            states: None,
            static_routes_only: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            virtual_gateway_ids: None,
            vpn_connection_ids: None,
        }
    }
}
