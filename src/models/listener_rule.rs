/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.27
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// ListenerRule : Information about the listener rule.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListenerRule {
    /// The type of action for the rule (always `forward`).
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// A host-name pattern for the rule, with a maximum length of 128 characters. This host-name pattern supports maximum three wildcards, and must not contain any special characters except [-.?].
    #[serde(rename = "HostNamePattern", skip_serializing_if = "Option::is_none")]
    pub host_name_pattern: Option<String>,
    /// The ID of the listener.
    #[serde(rename = "ListenerId", skip_serializing_if = "Option::is_none")]
    pub listener_id: Option<i32>,
    /// The ID of the listener rule.
    #[serde(rename = "ListenerRuleId", skip_serializing_if = "Option::is_none")]
    pub listener_rule_id: Option<i32>,
    /// A human-readable name for the listener rule.
    #[serde(rename = "ListenerRuleName", skip_serializing_if = "Option::is_none")]
    pub listener_rule_name: Option<String>,
    /// A path pattern for the rule, with a maximum length of 128 characters. This path pattern supports maximum three wildcards, and must not contain any special characters except [_-.$/~&quot;'@:+?].
    #[serde(rename = "PathPattern", skip_serializing_if = "Option::is_none")]
    pub path_pattern: Option<String>,
    /// The priority level of the listener rule, between `1` and `19999` both included. Each rule must have a unique priority level. Otherwise, an error is returned.
    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// The IDs of the backend VMs.
    #[serde(rename = "VmIds", skip_serializing_if = "Option::is_none")]
    pub vm_ids: Option<Vec<String>>,
}

impl ListenerRule {
    /// Information about the listener rule.
    pub fn new() -> ListenerRule {
        ListenerRule {
            action: None,
            host_name_pattern: None,
            listener_id: None,
            listener_rule_id: None,
            listener_rule_name: None,
            path_pattern: None,
            priority: None,
            vm_ids: None,
        }
    }
}
