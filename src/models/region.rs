/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.21
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Region : Information about the Region.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Region {
    /// The hostname of the gateway to access the Region.
    #[serde(rename = "Endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The administrative name of the Region.
    #[serde(rename = "RegionName", skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

impl Region {
    /// Information about the Region.
    pub fn new() -> Region {
        Region {
            endpoint: None,
            region_name: None,
        }
    }
}
