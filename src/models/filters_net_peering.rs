/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersNetPeering : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersNetPeering {
    /// The account IDs of the owners of the peer Nets.
    #[serde(
        rename = "AccepterNetAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepter_net_account_ids: Option<Vec<String>>,
    /// The IP ranges of the peer Nets, in CIDR notation (for example, `10.0.0.0/24`).
    #[serde(
        rename = "AccepterNetIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub accepter_net_ip_ranges: Option<Vec<String>>,
    /// The IDs of the peer Nets.
    #[serde(rename = "AccepterNetNetIds", skip_serializing_if = "Option::is_none")]
    pub accepter_net_net_ids: Option<Vec<String>>,
    /// The IDs of the Net peering connections.
    #[serde(rename = "NetPeeringIds", skip_serializing_if = "Option::is_none")]
    pub net_peering_ids: Option<Vec<String>>,
    /// The account IDs of the owners of the peer Nets.
    #[serde(
        rename = "SourceNetAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_net_account_ids: Option<Vec<String>>,
    /// The IP ranges of the peer Nets.
    #[serde(rename = "SourceNetIpRanges", skip_serializing_if = "Option::is_none")]
    pub source_net_ip_ranges: Option<Vec<String>>,
    /// The IDs of the peer Nets.
    #[serde(rename = "SourceNetNetIds", skip_serializing_if = "Option::is_none")]
    pub source_net_net_ids: Option<Vec<String>>,
    /// Additional information about the states of the Net peering connections.
    #[serde(rename = "StateMessages", skip_serializing_if = "Option::is_none")]
    pub state_messages: Option<Vec<String>>,
    /// The states of the Net peering connections (`pending-acceptance` \\| `active` \\| `rejected` \\| `failed` \\| `expired` \\| `deleted`).
    #[serde(rename = "StateNames", skip_serializing_if = "Option::is_none")]
    pub state_names: Option<Vec<String>>,
    /// The keys of the tags associated with the Net peering connections.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the Net peering connections.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the Net peering connections, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersNetPeering {
    /// One or more filters.
    pub fn new() -> FiltersNetPeering {
        FiltersNetPeering {
            accepter_net_account_ids: None,
            accepter_net_ip_ranges: None,
            accepter_net_net_ids: None,
            net_peering_ids: None,
            source_net_account_ids: None,
            source_net_ip_ranges: None,
            source_net_net_ids: None,
            state_messages: None,
            state_names: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
