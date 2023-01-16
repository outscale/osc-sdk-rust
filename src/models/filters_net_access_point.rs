/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersNetAccessPoint : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersNetAccessPoint {
    /// The IDs of the Net access points.
    #[serde(rename = "NetAccessPointIds", skip_serializing_if = "Option::is_none")]
    pub net_access_point_ids: Option<Vec<String>>,
    /// The IDs of the Nets.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The names of the services. For more information, see [ReadNetAccessPointServices](#readnetaccesspointservices).
    #[serde(rename = "ServiceNames", skip_serializing_if = "Option::is_none")]
    pub service_names: Option<Vec<String>>,
    /// The states of the Net access points (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "States", skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// The keys of the tags associated with the Net access points.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the Net access points.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the Net access points, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersNetAccessPoint {
    /// One or more filters.
    pub fn new() -> FiltersNetAccessPoint {
        FiltersNetAccessPoint {
            net_access_point_ids: None,
            net_ids: None,
            service_names: None,
            states: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
