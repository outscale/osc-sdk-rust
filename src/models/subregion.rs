/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Subregion : Information about the Subregion.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Subregion {
    /// The location code of the Subregion.
    #[serde(rename = "LocationCode", skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
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
            location_code: None,
            region_name: None,
            state: None,
            subregion_name: None,
        }
    }
}
