/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiAccessRule : Information about the API access rule.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiAccessRule {
    ///  The ID of the API access rule.
    #[serde(rename = "ApiAccessRuleId", skip_serializing_if = "Option::is_none")]
    pub api_access_rule_id: Option<String>,
    /// One or more IDs of Client Certificate Authorities (CAs) used for the API access rule.
    #[serde(rename = "CaIds", skip_serializing_if = "Option::is_none")]
    pub ca_ids: Option<Vec<String>>,
    /// One or more Client Certificate Common Names (CNs).
    #[serde(rename = "Cns", skip_serializing_if = "Option::is_none")]
    pub cns: Option<Vec<String>>,
    /// The description of the API access rule.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// One or more IP ranges used for the API access rule, in CIDR notation (for example, `192.0.2.0/16`).
    #[serde(rename = "IpRanges", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
}

impl ApiAccessRule {
    /// Information about the API access rule.
    pub fn new() -> ApiAccessRule {
        ApiAccessRule {
            api_access_rule_id: None,
            ca_ids: None,
            cns: None,
            description: None,
            ip_ranges: None,
        }
    }
}
