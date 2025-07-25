/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. > > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html). # Pagination Tutorial You can learn more about the pagination methods for read calls in the dedicated [pagination tutorial](https://docs.outscale.com/en/userguide/Tutorial-Paginating-an-API-Request.html). # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.35.4
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// Nic : Information about the NIC.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Nic {
    /// The account ID of the owner of the NIC.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The description of the NIC.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// (Net only) If true, the source/destination check is enabled. If false, it is disabled.
    #[serde(
        rename = "IsSourceDestChecked",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_source_dest_checked: Option<bool>,
    #[serde(rename = "LinkNic", skip_serializing_if = "Option::is_none")]
    pub link_nic: Option<Box<crate::models::LinkNic>>,
    #[serde(rename = "LinkPublicIp", skip_serializing_if = "Option::is_none")]
    pub link_public_ip: Option<Box<crate::models::LinkPublicIp>>,
    /// The Media Access Control (MAC) address of the NIC.
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// The ID of the Net for the NIC.
    #[serde(rename = "NetId", skip_serializing_if = "Option::is_none")]
    pub net_id: Option<String>,
    /// The ID of the NIC.
    #[serde(rename = "NicId", skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    /// The name of the private DNS.
    #[serde(rename = "PrivateDnsName", skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// The private IPs of the NIC.
    #[serde(rename = "PrivateIps", skip_serializing_if = "Option::is_none")]
    pub private_ips: Option<Vec<crate::models::PrivateIp>>,
    /// One or more IDs of security groups for the NIC.
    #[serde(rename = "SecurityGroups", skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<crate::models::SecurityGroupLight>>,
    /// The state of the NIC (`available` \\| `attaching` \\| `in-use` \\| `detaching`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the Subnet.
    #[serde(rename = "SubnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// The Subregion in which the NIC is located.
    #[serde(rename = "SubregionName", skip_serializing_if = "Option::is_none")]
    pub subregion_name: Option<String>,
    /// One or more tags associated with the NIC.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
}

impl Nic {
    /// Information about the NIC.
    pub fn new() -> Nic {
        Nic {
            account_id: None,
            description: None,
            is_source_dest_checked: None,
            link_nic: None,
            link_public_ip: None,
            mac_address: None,
            net_id: None,
            nic_id: None,
            private_dns_name: None,
            private_ips: None,
            security_groups: None,
            state: None,
            subnet_id: None,
            subregion_name: None,
            tags: None,
        }
    }
}
