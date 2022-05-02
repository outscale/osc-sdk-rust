/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.19
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersDirectLinkInterface : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersDirectLinkInterface {
    /// The IDs of the DirectLinks.
    #[serde(rename = "DirectLinkIds", skip_serializing_if = "Option::is_none")]
    pub direct_link_ids: Option<Vec<String>>,
    /// The IDs of the DirectLink interfaces.
    #[serde(
        rename = "DirectLinkInterfaceIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_link_interface_ids: Option<Vec<String>>,
}

impl FiltersDirectLinkInterface {
    /// One or more filters.
    pub fn new() -> FiltersDirectLinkInterface {
        FiltersDirectLinkInterface {
            direct_link_ids: None,
            direct_link_interface_ids: None,
        }
    }
}
