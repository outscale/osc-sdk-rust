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
pub struct CreateDhcpOptionsRequest {
    /// Specify a domain name (for example, `MyCompany.com`). You can specify only one domain name. You must specify at least one of the following parameters: `DomainName`, `DomainNameServers`, `LogServers`, or `NtpServers`.
    #[serde(rename = "DomainName", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// The IPs of domain name servers. If no IPs are specified, the `OutscaleProvidedDNS` value is set by default. You must specify at least one of the following parameters: `DomainName`, `DomainNameServers`, `LogServers`, or `NtpServers`.
    #[serde(rename = "DomainNameServers", skip_serializing_if = "Option::is_none")]
    pub domain_name_servers: Option<Vec<String>>,
    /// If true, checks whether you have the required permissions to perform the action.
    #[serde(rename = "DryRun", skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// The IPs of the log servers. You must specify at least one of the following parameters: `DomainName`, `DomainNameServers`, `LogServers`, or `NtpServers`.
    #[serde(rename = "LogServers", skip_serializing_if = "Option::is_none")]
    pub log_servers: Option<Vec<String>>,
    /// The IPs of the Network Time Protocol (NTP) servers. You must specify at least one of the following parameters: `DomainName`, `DomainNameServers`, `LogServers`, or `NtpServers`.
    #[serde(rename = "NtpServers", skip_serializing_if = "Option::is_none")]
    pub ntp_servers: Option<Vec<String>>,
}

impl CreateDhcpOptionsRequest {
    pub fn new() -> CreateDhcpOptionsRequest {
        CreateDhcpOptionsRequest {
            domain_name: None,
            domain_name_servers: None,
            dry_run: None,
            log_servers: None,
            ntp_servers: None,
        }
    }
}
