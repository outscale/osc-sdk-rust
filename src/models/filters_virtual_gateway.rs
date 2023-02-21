/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.25
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersVirtualGateway : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersVirtualGateway {
    /// The types of the virtual gateways (only `ipsec.1` is supported).
    #[serde(rename = "ConnectionTypes", skip_serializing_if = "Option::is_none")]
    pub connection_types: Option<Vec<String>>,
    /// The IDs of the Nets the virtual gateways are attached to.
    #[serde(rename = "LinkNetIds", skip_serializing_if = "Option::is_none")]
    pub link_net_ids: Option<Vec<String>>,
    /// The current states of the attachments between the virtual gateways and the Nets (`attaching` \\| `attached` \\| `detaching` \\| `detached`).
    #[serde(rename = "LinkStates", skip_serializing_if = "Option::is_none")]
    pub link_states: Option<Vec<String>>,
    /// The states of the virtual gateways (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the virtual gateways.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the virtual gateways.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the virtual gateways, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The IDs of the virtual gateways.
    #[serde(rename = "VirtualGatewayIds", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_ids: Option<Vec<String>>,
}

impl FiltersVirtualGateway {
    /// One or more filters.
    pub fn new() -> FiltersVirtualGateway {
        FiltersVirtualGateway {
            connection_types: None,
            link_net_ids: None,
            link_states: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
            virtual_gateway_ids: None,
        }
    }
}
