/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.26
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateSecurityGroupRuleRequest {
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The direction of the flow: `Inbound` or `Outbound`. You can specify `Outbound` for Nets only.
    #[serde(rename = "Flow")]
    pub flow: String,
    /// The beginning of the port range for the TCP and UDP protocols, or an ICMP type number. If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters.
    #[serde(rename = "FromPortRange", skip_serializing_if = "Option::is_none")]
    pub from_port_range: Option<i32>,
    /// The IP protocol name (`tcp`, `udp`, `icmp`, or `-1` for all protocols). By default, `-1`. In a Net, this can also be an IP protocol number. For more information, see the [IANA.org website](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml). If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters.
    #[serde(rename = "IpProtocol", skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,
    /// The IP range for the security group rule, in CIDR notation (for example, 10.0.0.0/16). If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters.
    #[serde(rename = "IpRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// Information about the security group rule to create. If you specify this parent parameter and its subparameters, you cannot specify the following parent parameters: `FromPortRange`, `IpProtocol`, `IpRange`, and `ToPortRange`.
    #[serde(rename = "Rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::SecurityGroupRule>>,
    /// The account ID that owns the source or destination security group specified in the `SecurityGroupNameToLink` parameter.
    #[serde(
        rename = "SecurityGroupAccountIdToLink",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_group_account_id_to_link: Option<String>,
    /// The ID of the security group for which you want to create a rule.
    #[serde(rename = "SecurityGroupId")]
    pub security_group_id: String,
    /// The ID of a source or destination security group that you want to link to the security group of the rule.
    #[serde(
        rename = "SecurityGroupNameToLink",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_group_name_to_link: Option<String>,
    /// The end of the port range for the TCP and UDP protocols, or an ICMP code number. If you specify this parameter, you cannot specify the `Rules` parameter and its subparameters.
    #[serde(rename = "ToPortRange", skip_serializing_if = "Option::is_none")]
    pub to_port_range: Option<i32>,
}

impl CreateSecurityGroupRuleRequest {
    pub fn new(flow: String, security_group_id: String) -> CreateSecurityGroupRuleRequest {
        CreateSecurityGroupRuleRequest {
            dry_run: None,
            flow,
            from_port_range: None,
            ip_protocol: None,
            ip_range: None,
            rules: None,
            security_group_account_id_to_link: None,
            security_group_id,
            security_group_name_to_link: None,
            to_port_range: None,
        }
    }
}
