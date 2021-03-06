/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateApiAccessRuleRequest {
    /// The ID of the API access rule you want to update.
    #[serde(rename = "ApiAccessRuleId")]
    pub api_access_rule_id: String,
    /// One or more IDs of Client Certificate Authorities (CAs).
    #[serde(rename = "CaIds", skip_serializing_if = "Option::is_none")]
    pub ca_ids: Option<Vec<String>>,
    /// One or more Client Certificate Common Names (CNs).
    #[serde(rename = "Cns", skip_serializing_if = "Option::is_none")]
    pub cns: Option<Vec<String>>,
    /// A new description for the API access rule.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// One or more IP ranges, in CIDR notation (for example, `192.0.2.0/16`).
    #[serde(rename = "IpRanges", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
}

impl UpdateApiAccessRuleRequest {
    pub fn new(api_access_rule_id: String) -> UpdateApiAccessRuleRequest {
        UpdateApiAccessRuleRequest {
            api_access_rule_id,
            ca_ids: None,
            cns: None,
            description: None,
            dry_run: None,
            ip_ranges: None,
        }
    }
}
