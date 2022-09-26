/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).
 *
 * The version of the OpenAPI document: 1.22
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityGroupRule : Information about the security group rule.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityGroupRule {
    /// The beginning of the port range for the TCP and UDP protocols, or an ICMP type number.
    #[serde(rename = "FromPortRange", skip_serializing_if = "Option::is_none")]
    pub from_port_range: Option<i32>,
    /// The IP protocol name (`tcp`, `udp`, `icmp`, or `-1` for all protocols). By default, `-1`. In a Net, this can also be an IP protocol number. For more information, see the [IANA.org website](https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml).
    #[serde(rename = "IpProtocol", skip_serializing_if = "Option::is_none")]
    pub ip_protocol: Option<String>,
    /// One or more IP ranges for the security group rules, in CIDR notation (for example, `10.0.0.0/16`).
    #[serde(rename = "IpRanges", skip_serializing_if = "Option::is_none")]
    pub ip_ranges: Option<Vec<String>>,
    /// Information about one or more members of a security group.
    #[serde(
        rename = "SecurityGroupsMembers",
        skip_serializing_if = "Option::is_none"
    )]
    pub security_groups_members: Option<Vec<crate::models::SecurityGroupsMember>>,
    /// One or more service IDs to allow traffic from a Net to access the corresponding OUTSCALE services. For more information, see [ReadNetAccessPointServices](#readnetaccesspointservices).
    #[serde(rename = "ServiceIds", skip_serializing_if = "Option::is_none")]
    pub service_ids: Option<Vec<String>>,
    /// The end of the port range for the TCP and UDP protocols, or an ICMP code number.
    #[serde(rename = "ToPortRange", skip_serializing_if = "Option::is_none")]
    pub to_port_range: Option<i32>,
}

impl SecurityGroupRule {
    /// Information about the security group rule.
    pub fn new() -> SecurityGroupRule {
        SecurityGroupRule {
            from_port_range: None,
            ip_protocol: None,
            ip_ranges: None,
            security_groups_members: None,
            service_ids: None,
            to_port_range: None,
        }
    }
}
