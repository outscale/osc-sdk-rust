/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.24
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateListenerRuleRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// A host-name pattern for the rule, with a maximum length of 128 characters. This host-name pattern supports maximum three wildcards, and must not contain any special characters except [-.?].
    #[serde(rename = "HostPattern", skip_serializing_if = "Option::is_none")]
    pub host_pattern: Option<String>,
    /// The name of the listener rule.
    #[serde(rename = "ListenerRuleName")]
    pub listener_rule_name: String,
    /// A path pattern for the rule, with a maximum length of 128 characters. This path pattern supports maximum three wildcards, and must not contain any special characters except [_-.$/~&quot;'@:+?].
    #[serde(rename = "PathPattern", skip_serializing_if = "Option::is_none")]
    pub path_pattern: Option<String>,
}

impl UpdateListenerRuleRequest {
    pub fn new(listener_rule_name: String) -> UpdateListenerRuleRequest {
        UpdateListenerRuleRequest {
            dry_run: None,
            host_pattern: None,
            listener_rule_name,
            path_pattern: None,
        }
    }
}
