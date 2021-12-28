/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersClientGateway : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersClientGateway {
    /// The Border Gateway Protocol (BGP) Autonomous System Numbers (ASNs) of the connections.
    #[serde(rename = "BgpAsns", skip_serializing_if = "Option::is_none")]
    pub bgp_asns: Option<Vec<i32>>,
    /// The IDs of the client gateways.
    #[serde(rename = "ClientGatewayIds", skip_serializing_if = "Option::is_none")]
    pub client_gateway_ids: Option<Vec<String>>,
    /// The types of communication tunnels used by the client gateways (only `ipsec.1` is supported).
    #[serde(rename = "ConnectionTypes", skip_serializing_if = "Option::is_none")]
    pub connection_types: Option<Vec<String>>,
    /// The public IPv4 addresses of the client gateways.
    #[serde(rename = "PublicIps", skip_serializing_if = "Option::is_none")]
    pub public_ips: Option<Vec<String>>,
    /// The states of the client gateways (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the client gateways.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the client gateways.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the client gateways, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersClientGateway {
    /// One or more filters.
    pub fn new() -> FiltersClientGateway {
        FiltersClientGateway {
            bgp_asns: None,
            client_gateway_ids: None,
            connection_types: None,
            public_ips: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
