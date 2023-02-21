/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersNet : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersNet {
    /// The IDs of the DHCP options sets.
    #[serde(rename = "DhcpOptionsSetIds", skip_serializing_if = "Option::is_none")]
    pub dhcp_options_set_ids: Option<Vec<String>>,
    /// The IP ranges for the Nets, in CIDR notation (for example, `10.0.0.0/16`).
    #[serde(rename = "IpRanges", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
    /// If true, the Net used is the default one.
    #[serde(rename = "IsDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The IDs of the Nets.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The states of the Nets (`pending` \\| `available` \\| `deleted`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the Nets.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the Nets.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the Nets, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersNet {
    /// One or more filters.
    pub fn new() -> FiltersNet {
        FiltersNet {
            dhcp_options_set_ids: None,
            ip_ranges: None,
            is_default: None,
            net_ids: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
