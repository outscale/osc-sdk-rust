/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> You can learn more about errors returned by the API in the dedicated [errors page](api/errors).<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/OUTSCALE-APIs-Reference.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html). ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages. ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).
 *
 * The version of the OpenAPI document: 1.28.7
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// DirectLinkInterfaces : Information about the DirectLink interfaces.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DirectLinkInterfaces {
    /// The account ID of the owner of the DirectLink interface.
    #[serde(rename = "AccountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The BGP (Border Gateway Protocol) ASN (Autonomous System Number) on the customer's side of the DirectLink interface.
    #[serde(rename = "BgpAsn", skip_serializing_if = "Option::is_none")]
    pub bgp_asn: Option<i32>,
    /// The BGP authentication key.
    #[serde(rename = "BgpKey", skip_serializing_if = "Option::is_none")]
    pub bgp_key: Option<String>,
    /// The IP on the customer's side of the DirectLink interface.
    #[serde(rename = "ClientPrivateIp", skip_serializing_if = "Option::is_none")]
    pub client_private_ip: Option<String>,
    /// The ID of the DirectLink.
    #[serde(rename = "DirectLinkId", skip_serializing_if = "Option::is_none")]
    pub direct_link_id: Option<String>,
    /// The ID of the DirectLink interface.
    #[serde(
        rename = "DirectLinkInterfaceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_link_interface_id: Option<String>,
    /// The name of the DirectLink interface.
    #[serde(
        rename = "DirectLinkInterfaceName",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_link_interface_name: Option<String>,
    /// The type of the DirectLink interface (always `private`).
    #[serde(rename = "InterfaceType", skip_serializing_if = "Option::is_none")]
    pub interface_type: Option<String>,
    /// The datacenter where the DirectLink interface is located.
    #[serde(rename = "Location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The maximum transmission unit (MTU) of the DirectLink interface, in bytes (always `1500`).
    #[serde(rename = "Mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<i32>,
    /// The IP on the OUTSCALE side of the DirectLink interface.
    #[serde(rename = "OutscalePrivateIp", skip_serializing_if = "Option::is_none")]
    pub outscale_private_ip: Option<String>,
    /// The state of the DirectLink interface (`pending` \\| `available` \\| `deleting` \\| `deleted` \\| `confirming` \\| `rejected` \\| `expired`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ID of the target virtual gateway.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// The VLAN number associated with the DirectLink interface.
    #[serde(rename = "Vlan", skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i32>,
}

impl DirectLinkInterfaces {
    /// Information about the DirectLink interfaces.
    pub fn new() -> DirectLinkInterfaces {
        DirectLinkInterfaces {
            account_id: None,
            bgp_asn: None,
            bgp_key: None,
            client_private_ip: None,
            direct_link_id: None,
            direct_link_interface_id: None,
            direct_link_interface_name: None,
            interface_type: None,
            location: None,
            mtu: None,
            outscale_private_ip: None,
            state: None,
            virtual_gateway_id: None,
            vlan: None,
        }
    }
}
