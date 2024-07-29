/*
 * 3DS OUTSCALE API
 *
 * Welcome to the OUTSCALE API documentation.<br /> The OUTSCALE API enables you to manage your resources in the OUTSCALE Cloud. This documentation describes the different actions available along with code examples.<br /><br /> Throttling: To protect against overloads, the number of identical requests allowed in a given time period is limited.<br /> Brute force: To protect against brute force attacks, the number of failed authentication attempts in a given time period is limited.<br /><br /> Note that the OUTSCALE Cloud is compatible with Amazon Web Services (AWS) APIs, but there are [differences in resource names](https://docs.outscale.com/en/userguide/About-the-APIs.html) between AWS and the OUTSCALE API.<br /> You can also manage your resources using the [Cockpit](https://docs.outscale.com/en/userguide/About-Cockpit.html) web interface.<br /><br /> An OpenAPI description of the OUTSCALE API is also available in this [GitHub repository](https://github.com/outscale/osc-api).<br /> # Authentication Schemes ### Access Key/Secret Key The main way to authenticate your requests to the OUTSCALE API is to use an access key and a secret key.<br /> The mechanism behind this is based on AWS Signature Version 4, whose technical implementation details are described in [Signature of API Requests](https://docs.outscale.com/en/userguide/Signature-of-API-Requests.html).<br /><br /> In practice, the way to specify your access key and secret key depends on the tool or SDK you want to use to interact with the API.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify your access key, secret key, and the Region of your account. > 2. You then specify the `--profile` option when executing OSC CLI commands. >  > For more information, see [Installing and Configuring OSC CLI](https://docs.outscale.com/en/userguide/Installing-and-Configuring-OSC-CLI.html).  See the code samples in each section of this documentation for specific examples in different programming languages.<br /> For more information about access keys, see [About Access Keys](https://docs.outscale.com/en/userguide/About-Access-Keys.html).  > If you try to sign requests with an invalid access key four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### Login/Password For certain API actions, you can also use basic authentication with the login (email address) and password of your TINA account.<br /> This is useful only in special circumstances, for example if you do not know your access key/secret key and want to retrieve them programmatically.<br /> In most cases, however, you can use the Cockpit web interface to retrieve them.<br />  > For example, if you use OSC CLI: > 1. You need to create an **~/.osc/config.json** file to specify the Region of your account, but you leave the access key value and secret key value empty (`&quot;&quot;`). > 2. You then specify the `--profile`, `--authentication-method`, `--login`, and `--password` options when executing OSC CLI commands.  See the code samples in each section of this documentation for specific examples in different programming languages.  > If you try to sign requests with an invalid password four times in a row, further authentication attempts will be prevented for 1 minute. This lockout time increases 1 minute every four failed attempts, for up to 10 minutes.  ### No Authentication A few API actions do not require any authentication. They are indicated as such in this documentation.<br /> ### Other Security Mechanisms In parallel with the authentication schemes, you can add other security mechanisms to your OUTSCALE account, for example to restrict API requests by IP or other criteria.<br /> For more information, see [Managing Your API Accesses](https://docs.outscale.com/en/userguide/Managing-Your-API-Accesses.html).<br /> # Error Codes Reference You can learn more about errors returned by the API in the dedicated [errors page](api-errors.html).
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@outscale.com
 * Generated by: https://openapi-generator.tech
 */

/// VpnConnection : Information about a VPN connection.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VpnConnection {
    /// Example configuration for the client gateway.
    #[serde(
        rename = "ClientGatewayConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_gateway_configuration: Option<String>,
    /// The ID of the client gateway used on the client end of the connection.
    #[serde(rename = "ClientGatewayId", skip_serializing_if = "Option::is_none")]
    pub client_gateway_id: Option<String>,
    /// The type of VPN connection (always `ipsec.1`).
    #[serde(rename = "ConnectionType", skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// Information about one or more static routes associated with the VPN connection, if any.
    #[serde(rename = "Routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<crate::models::RouteLight>>,
    /// The state of the VPN connection (`pending` \\| `available` \\| `deleting` \\| `deleted`).
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// If false, the VPN connection uses dynamic routing with Border Gateway Protocol (BGP). If true, routing is controlled using static routes. For more information about how to create and delete static routes, see [CreateVpnConnectionRoute](#createvpnconnectionroute) and [DeleteVpnConnectionRoute](#deletevpnconnectionroute).
    #[serde(rename = "StaticRoutesOnly", skip_serializing_if = "Option::is_none")]
    pub static_routes_only: Option<bool>,
    /// One or more tags associated with the VPN connection.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::ResourceTag>>,
    /// Information about the current state of one or more of the VPN tunnels.
    #[serde(rename = "VgwTelemetries", skip_serializing_if = "Option::is_none")]
    pub vgw_telemetries: Option<Vec<crate::models::VgwTelemetry>>,
    /// The ID of the virtual gateway used on the OUTSCALE end of the connection.
    #[serde(rename = "VirtualGatewayId", skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// The ID of the VPN connection.
    #[serde(rename = "VpnConnectionId", skip_serializing_if = "Option::is_none")]
    pub vpn_connection_id: Option<String>,
    #[serde(rename = "VpnOptions", skip_serializing_if = "Option::is_none")]
    pub vpn_options: Option<Box<crate::models::VpnOptions>>,
}

impl VpnConnection {
    /// Information about a VPN connection.
    pub fn new() -> VpnConnection {
        VpnConnection {
            client_gateway_configuration: None,
            client_gateway_id: None,
            connection_type: None,
            routes: None,
            state: None,
            static_routes_only: None,
            tags: None,
            vgw_telemetries: None,
            virtual_gateway_id: None,
            vpn_connection_id: None,
            vpn_options: None,
        }
    }
}
