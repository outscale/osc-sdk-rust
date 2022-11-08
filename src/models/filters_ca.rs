/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersCa : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersCa {
    /// The fingerprints of the CAs.
    #[serde(rename = "CaFingerprints", skip_serializing_if = "Option::is_none")]
    pub ca_fingerprints: Option<Vec<String>>,
    /// The IDs of the CAs.
    #[serde(rename = "CaIds", skip_serializing_if = "Option::is_none")]
    pub ca_ids: Option<Vec<String>>,
    /// The descriptions of the CAs.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
}

impl FiltersCa {
    /// One or more filters.
    pub fn new() -> FiltersCa {
        FiltersCa {
            ca_fingerprints: None,
            ca_ids: None,
            descriptions: None,
        }
    }
}
