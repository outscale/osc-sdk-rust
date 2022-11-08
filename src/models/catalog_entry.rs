/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.23
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// CatalogEntry : Information about the catalog entry.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// The category of the catalog entry (for example, `network`).
    #[serde(rename = "Category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// When returned and equal to `PER_MONTH`, the price of the catalog entry is calculated on a monthly basis.
    #[serde(rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    /// The API call associated with the catalog entry (for example, `CreateVms` or `RunInstances`).
    #[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// The service associated with the catalog entry (`TinaOS-FCU`, `TinaOS-LBU`, `TinaOS-DirectLink`, or `TinaOS-OOS`).
    #[serde(rename = "Service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The Subregion associated with the catalog entry.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// The description of the catalog entry.
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of resource associated with the catalog entry.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The unit price of the catalog entry, in the currency of the catalog of the Region where the API method was used.
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f32>,
}

impl CatalogEntry {
    /// Information about the catalog entry.
    pub fn new() -> CatalogEntry {
        CatalogEntry {
            category: None,
            flags: None,
            operation: None,
            service: None,
            subregion_name: None,
            title: None,
            _type: None,
            unit_price: None,
        }
    }
}
