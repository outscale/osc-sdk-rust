/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html).<br /><br />  You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.18
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Subregion : Information about the Subregion.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subregion {
    /// The name of the Region containing the Subregion.
    #[serde(rename = "RegionName", skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    /// The state of the Subregion (`available` \\| `information` \\| `impaired` \\| `unavailable`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The name of the Subregion.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
}

impl Subregion {
    /// Information about the Subregion.
    pub fn new() -> Subregion {
        Subregion {
            region_name: None,
            state: None,
            subregion_name: None,
        }
    }
}
