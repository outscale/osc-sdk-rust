/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /><br />  The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br />  Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but some resources have different names in AWS than in the OUTSCALE API. You can find a list of the differences [here](https://wiki.outscale.net/display/EN/3DS+OUTSCALE+APIs+Reference).<br /><br />  You can also manage your resources using the [Cockpit](https://wiki.outscale.net/display/EN/About+Cockpit) web interface.
 *
 * The version of the OpenAPI document: 1.16
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersQuota : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersQuota {
    /// The group names of the quotas.
    #[serde(rename = "Collections", skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<String>>,
    /// The names of the quotas.
    #[serde(rename = "QuotaNames", skip_serializing_if = "Option::is_none")]
    pub quota_names: Option<Vec<String>>,
    /// The resource IDs if these are resource-specific quotas, `global` if they are not.
    #[serde(rename = "QuotaTypes", skip_serializing_if = "Option::is_none")]
    pub quota_types: Option<Vec<String>>,
    /// The description of the quotas.
    #[serde(rename = "ShortDescriptions", skip_serializing_if = "Option::is_none")]
    pub short_descriptions: Option<Vec<String>>,
}

impl FiltersQuota {
    /// One or more filters.
    pub fn new() -> FiltersQuota {
        FiltersQuota {
            collections: None,
            quota_names: None,
            quota_types: None,
            short_descriptions: None,
        }
    }
}
