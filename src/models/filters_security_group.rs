/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersSecurityGroup : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersSecurityGroup {
    /// The account IDs of the owners of the security groups.
    #[serde(rename = "AccountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// The descriptions of the security groups.
    #[serde(rename = "Descriptions", skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// The account IDs that have been granted permissions.
    #[serde(
        rename = "InboundRuleAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_account_ids: Option<Vec<String>>,
    /// The beginnings of the port ranges for the TCP and UDP protocols, or the ICMP type numbers.
    #[serde(
        rename = "InboundRuleFromPortRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_from_port_ranges: Option<Vec<i32>>,
    /// The IP ranges that have been granted permissions, in CIDR notation (for example, `10.0.0.0/24`).
    #[serde(
        rename = "InboundRuleIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_ip_ranges: Option<Vec<String>>,
    /// The IP protocols for the permissions (`tcp` \\| `udp` \\| `icmp`, or a protocol number, or `-1` for all protocols).
    #[serde(
        rename = "InboundRuleProtocols",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_protocols: Option<Vec<String>>,
    /// The IDs of the security groups that have been granted permissions.
    #[serde(
        rename = "InboundRuleSecurityGroupIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_security_group_ids: Option<Vec<String>>,
    /// The names of the security groups that have been granted permissions.
    #[serde(
        rename = "InboundRuleSecurityGroupNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_security_group_names: Option<Vec<String>>,
    /// The ends of the port ranges for the TCP and UDP protocols, or the ICMP code numbers.
    #[serde(
        rename = "InboundRuleToPortRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_rule_to_port_ranges: Option<Vec<i32>>,
    /// The IDs of the Nets specified when the security groups were created.
    #[serde(rename = "NetIds", skip_serializing_if = "Option::is_none")]
    pub net_ids: Option<Vec<String>>,
    /// The account IDs that have been granted permissions.
    #[serde(
        rename = "OutboundRuleAccountIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_account_ids: Option<Vec<String>>,
    /// The beginnings of the port ranges for the TCP and UDP protocols, or the ICMP type numbers.
    #[serde(
        rename = "OutboundRuleFromPortRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_from_port_ranges: Option<Vec<i32>>,
    /// The IP ranges that have been granted permissions, in CIDR notation (for example, `10.0.0.0/24`).
    #[serde(
        rename = "OutboundRuleIpRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_ip_ranges: Option<Vec<String>>,
    /// The IP protocols for the permissions (`tcp` \\| `udp` \\| `icmp`, or a protocol number, or `-1` for all protocols).
    #[serde(
        rename = "OutboundRuleProtocols",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_protocols: Option<Vec<String>>,
    /// The IDs of the security groups that have been granted permissions.
    #[serde(
        rename = "OutboundRuleSecurityGroupIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_security_group_ids: Option<Vec<String>>,
    /// The names of the security groups that have been granted permissions.
    #[serde(
        rename = "OutboundRuleSecurityGroupNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_security_group_names: Option<Vec<String>>,
    /// The ends of the port ranges for the TCP and UDP protocols, or the ICMP code numbers.
    #[serde(
        rename = "OutboundRuleToPortRanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_rule_to_port_ranges: Option<Vec<i32>>,
    /// The IDs of the security groups.
    #[serde(rename = "SecurityGroupIds", skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// The names of the security groups.
    #[serde(rename = "SecurityGroupNames", skip_serializing_if = "Option::is_none")]
    pub security_group_names: Option<Vec<String>>,
    /// The keys of the tags associated with the security groups.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the security groups.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the security groups, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersSecurityGroup {
    /// One or more filters.
    pub fn new() -> FiltersSecurityGroup {
        FiltersSecurityGroup {
            account_ids: None,
            descriptions: None,
            inbound_rule_account_ids: None,
            inbound_rule_from_port_ranges: None,
            inbound_rule_ip_ranges: None,
            inbound_rule_protocols: None,
            inbound_rule_security_group_ids: None,
            inbound_rule_security_group_names: None,
            inbound_rule_to_port_ranges: None,
            net_ids: None,
            outbound_rule_account_ids: None,
            outbound_rule_from_port_ranges: None,
            outbound_rule_ip_ranges: None,
            outbound_rule_protocols: None,
            outbound_rule_security_group_ids: None,
            outbound_rule_security_group_names: None,
            outbound_rule_to_port_ranges: None,
            security_group_ids: None,
            security_group_names: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
