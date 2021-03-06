/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.
 *
 * The version of the OpenAPI document: 1.20
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// FiltersDhcpOptions : One or more filters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FiltersDhcpOptions {
    /// If true, lists all default DHCP options set. If false, lists all non-default DHCP options set.
    #[serde(rename = "Default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// The IDs of the DHCP options sets.
    #[serde(rename = "DhcpOptionsSetIds", skip_serializing_if = "Option::is_none")]
    pub dhcp_options_set_ids: Option<Vec<String>>,
    /// The IPs of the domain name servers used for the DHCP options sets.
    #[serde(rename = "DomainNameServers", skip_serializing_if = "Option::is_none")]
    pub domain_name_servers: Option<Vec<String>>,
    /// The domain names used for the DHCP options sets.
    #[serde(rename = "DomainNames", skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<String>>,
    /// The IPs of the log servers used for the DHCP options sets.
    #[serde(rename = "LogServers", skip_serializing_if = "Option::is_none")]
    pub log_servers: Option<Vec<String>>,
    /// The IPs of the Network Time Protocol (NTP) servers used for the DHCP options sets.
    #[serde(rename = "NtpServers", skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<Vec<String>>,
    /// The keys of the tags associated with the DHCP options sets.
    #[serde(rename = "TagKeys", skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
    /// The values of the tags associated with the DHCP options sets.
    #[serde(rename = "TagValues", skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
    /// The key/value combination of the tags associated with the DHCP options sets, in the following format: &quot;Filters&quot;:{&quot;Tags&quot;:[&quot;TAGKEY=TAGVALUE&quot;]}.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl FiltersDhcpOptions {
    /// One or more filters.
    pub fn new() -> FiltersDhcpOptions {
        FiltersDhcpOptions {
            default: None,
            dhcp_options_set_ids: None,
            domain_name_servers: None,
            domain_names: None,
            log_servers: None,
            ntp_servers: None,
            tag_keys: None,
            tag_values: None,
            tags: None,
        }
    }
}
